use std::io::{Read, Seek, SeekFrom};

use crate::error::{Error, Result};
use bytestream::StreamReader;

use super::ATR1;

impl ATR1{
    pub fn read_from<R: Read + Seek>(buffer: &mut R, order: bytestream::ByteOrder) -> Result<ATR1> {
        let block_start = buffer.stream_position()?;
        let mut magic = vec![0u8;4];
        buffer.read_exact(&mut magic)?;
        if magic != b"ATR1" {
            return Err(Error::Malformed)
        }
        let section_size = u32::read_from(buffer, order)?;
        buffer.seek(SeekFrom::Current(8))?;
        let string_amount = u64::read_from(buffer, order)?;
        buffer.seek(SeekFrom::Start(block_start+0x10+section_size as u64+(0x10-(section_size%0x10)) as u64))?;
        Ok(ATR1 { 
            magic: magic,
            section_size: section_size,
            string_amount: string_amount
        })
    }
}