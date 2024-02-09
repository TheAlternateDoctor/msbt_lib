use std::io::{Read, Seek, SeekFrom};
use bytestream::{ByteOrder, StreamReader};
use crate::{error::{Error, Result}, msbt::MSBTString};

#[derive(Debug, Clone)]
pub struct TXT2{
    magic: Vec::<u8>,
    section_size: u32,
    string_amount: u32,
    pub offsets: Vec<u32>,
    pub strings: Vec<Vec<u8>>
}

impl TXT2{
    pub fn read_from<R: Read + Seek>(buffer: &mut R, order: bytestream::ByteOrder) -> Result<TXT2> {
        let block_start = buffer.stream_position()?;
        let mut magic = vec![0u8;4];
        buffer.read_exact(&mut magic)?;
        if magic != b"TXT2" {
            return Err(Error::Malformed)
        }
        let section_size = u32::read_from(buffer, order)?;
        buffer.seek(SeekFrom::Current(8))?;
        let string_amount = u32::read_from(buffer, order)?;
        let mut offsets = Vec::<u32>::new();
        for _i in 0..string_amount {
            offsets.push(u32::read_from(buffer, order)?);
        }
        let strings = Self::get_strings(buffer, order, offsets.clone(), block_start+0x10)?;
        Ok(TXT2{
            magic: magic,
            section_size: section_size,
            string_amount: string_amount,
            offsets: offsets,
            strings: strings,
        })
    }

    fn get_strings<R:Read + Seek>(buffer: &mut R, order: bytestream::ByteOrder, offsets: Vec<u32>, start_pos: u64) -> Result<Vec<Vec<u8>>> {
        let mut strings = Vec::<Vec<u8>>::new();
        let mut start_offset = offsets[0];
        for offset in offsets{
            if offset != start_offset {
                // println!("{:#x}:{:#x}", start_pos,offset);
                buffer.seek(SeekFrom::Start(start_pos+start_offset as u64))?;
                let mut string = Vec::<u8>::new();
                for _i in 0..(offset-start_offset){
                    string.push(u8::read_from(buffer, order)?);
                }
                strings.push(string);
                start_offset = offset;
            }
        }
        buffer.seek(SeekFrom::Start(start_pos+start_offset as u64))?;
        let mut string = Vec::<u8>::new();
        buffer.read_to_end(&mut string)?;
        let mut padding = string.last().copied().unwrap();
        while padding != 0x00 {
            string.truncate(string.len()-1);
            padding = string.last().copied().unwrap();
        }
        strings.push(string);
        Ok(strings)
    }

    pub fn write_binary(msbt_strings: Vec<MSBTString>, order: bytestream::ByteOrder) -> Result<Vec<u8>> {
        let mut result = Vec::<u8>::new();
        let mut offsets = Vec::<u32>::new();
        let mut strings = Vec::<Vec<u8>>::new();
        let mut new_strings = msbt_strings.clone();
        new_strings.sort_by(|a, b| a.index.cmp(&b.index));
        //First offset
        let mut last_offset = msbt_strings.len() as u32*4+4;
        let mut section_size = 4 as u32; //amount of strings
        offsets.push(last_offset);
        for string in new_strings{
            last_offset = last_offset+string.string.len() as u32;
            section_size += string.string.len() as u32+4;
            strings.push(string.string);
            offsets.push(last_offset);
        }
        offsets.truncate(offsets.len()-1);
        //binary tiem
        result.append(&mut b"TXT2".to_vec());
        match order {
            ByteOrder::BigEndian => {
                result.append(&mut section_size.to_be_bytes().to_vec());
                result.append(&mut vec![0,0,0,0,0,0,0,0]);
                result.append(&mut (msbt_strings.len() as u32).to_be_bytes().to_vec());
                for offset in offsets{
                    result.append(&mut offset.to_be_bytes().to_vec());
                }
                for string in strings{
                    result.append(&mut string.clone());
                }
            }
            ByteOrder::LittleEndian => {
                result.append(&mut section_size.to_le_bytes().to_vec());
                result.append(&mut vec![0,0,0,0,0,0,0,0]);
                result.append(&mut (msbt_strings.len() as u32).to_le_bytes().to_vec());
                for offset in offsets{
                    result.append(&mut offset.to_le_bytes().to_vec());
                }
                for string in strings{
                    result.append(&mut string.clone());
                }
            }
        }

        Ok(result)
    }
}
