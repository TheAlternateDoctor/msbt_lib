use std::io::{Read, Seek, SeekFrom};

use crate::{error::{Error, Result}, msbt::MSBTString};
use bytestream::{ByteOrder, StreamReader};

#[derive(Debug, Clone)]
pub struct ATR1{ // Not enough data, since only Rhythm Heaven Megamix has been used for research.
    magic: Vec<u8>,
    section_size: u32,
    string_amount:u64
}

impl ATR1{
    pub fn read_from<R: Read + Seek>(buffer: &mut R, order: bytestream::ByteOrder) -> Result<ATR1> {
        println!("Extracting attributes...");
        let block_start = buffer.stream_position()?;
        let mut magic = vec![0u8;4];
        buffer.read_exact(&mut magic)?;
        if magic != b"ATR1" {
            buffer.seek(SeekFrom::Current(-4))?;
            println!("No ATR1 section, continuing...");
            return Ok(ATR1{ magic: "NONE".as_bytes().to_vec(), section_size: 0, string_amount: 0 });
        }
        let section_size = u32::read_from(buffer, order)?;
        buffer.seek(SeekFrom::Current(8))?;
        let string_amount = u64::read_from(buffer, order)?;
        buffer.seek(SeekFrom::Start(block_start+0x10+section_size as u64+(0x10-(section_size%0x10)) as u64))?;
        println!("Extracted attributes.");
        Ok(ATR1 { 
            magic: magic,
            section_size: section_size,
            string_amount: string_amount
        })
    }

    pub fn write_binary(msbt_strings: Vec<MSBTString>, order: bytestream::ByteOrder) -> Result<Vec<u8>>{
        println!("Formatting attributes...");
        let mut result = Vec::<u8>::new();
        let section_size = 8 as u32;
        //binary tiem
        result.append(&mut b"ATR1".to_vec());
        match order {
            ByteOrder::BigEndian => {
                result.append(&mut section_size.to_be_bytes().to_vec());
                result.append(&mut vec![0,0,0,0,0,0,0,0]);
                result.append(&mut (msbt_strings.len()).to_be_bytes().to_vec());
            }
            ByteOrder::LittleEndian => {
                result.append(&mut section_size.to_le_bytes().to_vec());
                result.append(&mut vec![0,0,0,0,0,0,0,0]);
                result.append(&mut (msbt_strings.len()).to_le_bytes().to_vec());
            }
        }
        let padding = 16 - result.len() %16;
        for _i in 0..padding{
            result.push(0xD0);
        }

        println!("Formated attributes.");
        Ok(result)
    }
}