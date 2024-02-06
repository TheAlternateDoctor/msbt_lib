use std::io::{Read, Seek, SeekFrom};
use bytestream::StreamReader;
use crate::error::{Error, Result};

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
}
