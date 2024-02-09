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
    pub string_index: u32
}

impl LBL1 {
    pub fn read_from<R: Read + Seek>(buffer: &mut R, order: bytestream::ByteOrder) -> Result<LBL1> {
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
                    label: String::from_utf8(string)?,
                    string_index: index
                });
            }
        }
        Ok(labels)
    }
    pub fn write_binary(msbt_strings: Vec<MSBTString>, order: bytestream::ByteOrder) -> Result<Vec<u8>> {
        let mut result = Vec::<u8>::new();
        let mut offsets = Vec::<LabelDef>::new();
        let mut strings = Vec::<u8>::new();
        let mut new_strings = msbt_strings.clone();
        let label_amount = msbt_strings.len();
        let  mut per_section = label_amount/101+1;
        let mut last_offset = 0x32C; //Always the same for MSBT

        while !new_strings.is_empty() {
            let mut label_def = LabelDef{ amount: 0, offset: last_offset };
            for _i in 0..per_section{
                if !new_strings.is_empty() {
                    label_def.amount += 1;
                    let mut string = new_strings.pop().unwrap();
                    let mut added_string = Vec::<u8>::new();
                    added_string.push(string.label.len() as u8);
                    added_string.append(&mut string.label.into_bytes());
                    added_string.append(&mut string.index.to_le_bytes().to_vec());
                    last_offset += added_string.len() as u32;
                    strings.append(&mut added_string);
                }
            }
            offsets.push(label_def);
        }
        //Pad out the last few blocks
        if offsets.len() < 101 {
            let last_label = offsets.pop().unwrap();
            for _i in offsets.len()..100{
                let label_def = LabelDef{ amount: 0, offset: last_label.offset };
                offsets.push(label_def);
            }
            offsets.push(last_label);
        }

        let section_size = 0x32C+strings.len() as u32;
        //binary tiem

        result.append(&mut b"LBL1".to_vec());
        match order {
            ByteOrder::BigEndian => {
                result.append(&mut section_size.to_be_bytes().to_vec());
                result.append(&mut vec![0,0,0,0,0,0,0,0]);
                result.append(&mut (101 as u32).to_be_bytes().to_vec());
                for offset in offsets{
                    result.append(&mut offset.amount.to_be_bytes().to_vec());
                    result.append(&mut offset.offset.to_be_bytes().to_vec());
                }
                    result.append(&mut strings.clone());
            }
            ByteOrder::LittleEndian => {
                result.append(&mut section_size.to_le_bytes().to_vec());
                result.append(&mut vec![0,0,0,0,0,0,0,0]);
                result.append(&mut (101 as u32).to_le_bytes().to_vec());
                for offset in offsets{
                    result.append(&mut offset.amount.to_le_bytes().to_vec());
                    result.append(&mut offset.offset.to_le_bytes().to_vec());
                }
                    result.append(&mut strings.clone());
            }
        }
        let padding = 16 - result.len() %16;
        for _i in 0..padding{
            result.push(0xD0);
        }

        Ok(result)
    }
}