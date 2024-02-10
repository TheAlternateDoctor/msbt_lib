use std::io::{Read, Seek, SeekFrom};
use bytestream::{ByteOrder, StreamReader};
use crate::{error::{Error, Result}, msbt::MSBTString};

#[derive(Debug, Clone)]
pub struct LBL1{
    magic: Vec::<u8>,
    section_size: u32,
    block_amount: u32,
    pub offsets: Vec<LabelDef>,
    pub labels: Vec<Label>
}

#[derive(Debug, Clone)]
pub struct LabelDef{
    amount: u32,
    offset: u32,
}

#[derive(Debug, Clone)]
pub struct Label{
    pub size: u8,
    pub label: String,
    pub string_index: u32,
    hash: u8
}

impl LBL1 {
    pub fn read_from<R: Read + Seek>(buffer: &mut R, order: bytestream::ByteOrder) -> Result<LBL1> {
        println!("Extracting labels...");
        let block_start = buffer.stream_position()?;
        let mut magic = vec![0u8;4];
        buffer.read_exact(&mut magic)?;
        if magic != b"LBL1" {
            return Err(Error::Malformed)
        }
        let section_size = u32::read_from(buffer, order)?;
        buffer.seek(SeekFrom::Current(8))?;
        let start_block = buffer.stream_position()?;
        let block_amount = u32::read_from(buffer, order)?;
        let label_defs = Self::get_offsets(buffer, order, block_amount)?;
        buffer.seek(SeekFrom::Start(start_block))?;
        let labels = Self::get_labels(buffer, order, label_defs.clone())?;
        buffer.seek(SeekFrom::Start(block_start+0x10+section_size as u64+(0x10-(section_size%0x10)) as u64))?;
        println!("Extracted labels.");
        Ok(LBL1{
            magic: magic,
            section_size: section_size,
            block_amount: block_amount,
            offsets: label_defs,
            labels: labels
        })
    }

    fn get_offsets<R:Read>(buffer: &mut R, order: bytestream::ByteOrder, amount: u32) -> Result<Vec<LabelDef>> {
        let mut offsets = Vec::<LabelDef>::new();
        for _i in 0..amount {
            let label_amount = u32::read_from(buffer, order)?;
            let offset = u32::read_from(buffer, order)?;
            offsets.push(LabelDef {
                amount: label_amount,
                offset: offset,
            })
        }
        Ok(offsets)
    }

    fn get_labels<R:Read+ Seek>(buffer: &mut R, order: bytestream::ByteOrder, label_defs: Vec<LabelDef>) -> Result<Vec<Label>> {
        let mut labels = Vec::<Label>::new();
        let start_pos = buffer.stream_position()?;
        for label_def in label_defs {
            buffer.seek(SeekFrom::Start(start_pos+label_def.offset as u64))?;
            for _i in 0..label_def.amount{
                let size = u8::read_from(buffer, order)?;
                let mut string = vec![0u8;size.into()];
                buffer.read_exact(&mut string)?;
                let index = u32::read_from(buffer, order)?;
                labels.push(Label {
                    size: size,
                    label: String::from_utf8(string.clone())?,
                    string_index: index,
                    hash: Self::calculate_hash(String::from_utf8(string)?) as u8
                });
            }
        }
        Ok(labels)
    }
    pub fn write_binary(msbt_strings: Vec<MSBTString>, order: bytestream::ByteOrder) -> Result<Vec<u8>> {
        println!("Formatting labels...");
        let mut result = Vec::<u8>::new();
        let mut labels = Vec::<Label>::new();
        let mut label_defs = vec![LabelDef{ amount: 0, offset: 0 };101];
        let mut labels_raw = Vec::<u8>::new();
        let base_offset = 0x32C; //Always the same for MSBT
        let mut label_defs_raw = Vec::<u8>::new();

        for string in msbt_strings {
            //Calculate the hash
            let hash = Self::calculate_hash(string.label.clone());
            let label = Label{
                size: string.label.len() as u8,
                label: string.label,
                string_index: string.index,
                hash: hash as u8,
            };
            labels.push(label);
        }
        //Now we prepare the offset array (vector of LabelDef)/the label array (raw)
        labels.sort_by(|a,b| a.hash.cmp(&b.hash));
        let mut label_def = LabelDef{amount: 0, offset: base_offset};
        let mut current_hash = 0;
        for label in labels{
            if label.hash != current_hash{
                label_defs.insert(current_hash as usize, label_def);
                label_def = LabelDef{amount: 0, offset: base_offset+labels_raw.len() as u32};
                current_hash = label.hash;
            }
            label_def.amount += 1;
            labels_raw.push(label.size);
            labels_raw.append(&mut label.label.as_bytes().to_vec());
            match order {
                ByteOrder::BigEndian => {labels_raw.append(&mut label.string_index.to_be_bytes().to_vec());}
                ByteOrder::LittleEndian => {labels_raw.append(&mut label.string_index.to_le_bytes().to_vec());}
            }
        }
        label_defs.insert(current_hash as usize, label_def);
        //Finally, we make the raw offset array
        let mut empties = 0u8;
        for i in 0..101{
            let label_def = label_defs.get(i).unwrap();
            if label_def.offset != 0 {
                for _i in 0..empties{
                    match order {
                        ByteOrder::BigEndian => {
                            label_defs_raw.append(&mut 0u32.to_be_bytes().to_vec());
                            label_defs_raw.append(&mut label_def.offset.to_be_bytes().to_vec());
                        }
                        ByteOrder::LittleEndian => {
                            label_defs_raw.append(&mut 0u32.to_le_bytes().to_vec());
                            label_defs_raw.append(&mut label_def.offset.to_le_bytes().to_vec());
                        }
                    }
                }
                match order {
                    ByteOrder::BigEndian => {
                        label_defs_raw.append(&mut label_def.amount.to_be_bytes().to_vec());
                        label_defs_raw.append(&mut label_def.offset.to_be_bytes().to_vec());
                    }
                    ByteOrder::LittleEndian => {
                        label_defs_raw.append(&mut label_def.amount.to_le_bytes().to_vec());
                        label_defs_raw.append(&mut label_def.offset.to_le_bytes().to_vec());
                    }
                }
                empties = 0;
            } else {
                empties += 1;
            }
        }

        //binary tiem
        let section_size = labels_raw.len() + label_defs_raw.len() + 4;
        result.append(&mut b"LBL1".to_vec());
        match order {
            ByteOrder::BigEndian => {
                result.append(&mut (section_size as u32).to_be_bytes().to_vec());
                result.append(&mut vec![0,0,0,0,0,0,0,0]);
                result.append(&mut (101 as u32).to_be_bytes().to_vec());
                result.append(&mut label_defs_raw.clone());
                result.append(&mut labels_raw.clone());
            }
            ByteOrder::LittleEndian => {
                result.append(&mut (section_size as u32).to_le_bytes().to_vec());
                result.append(&mut vec![0,0,0,0,0,0,0,0]);
                result.append(&mut (101 as u32).to_le_bytes().to_vec());
                result.append(&mut label_defs_raw.clone());
                result.append(&mut labels_raw.clone());
            }
        }
        let padding = 16 - result.len() %16;
        for _i in 0..padding{
            result.push(0xD0);
        }
        println!("Formated labels.");

        Ok(result)
    }

    fn calculate_hash(label: String) -> u64{
        let mut hash:u64 = 0;
        for char in label.as_bytes(){
            hash = hash.wrapping_mul(0x492) + (*char) as u64 ;
        }
        return (hash & 0xFFFFFFFF) % 101;
    }
}