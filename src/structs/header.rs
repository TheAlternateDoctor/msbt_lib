use std::io::{Read, Seek, SeekFrom};
use bytestream::{ByteOrder, StreamReader};
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Header{
    magic: Vec<u8>,
    pub endianness: bool,
    unk1: u16,
    unk2: u16, //Version? Always 01 03
    pub section_amount: u16,
    unk3: u16,
    filesize: u32,
}

impl Header{
    pub fn read_from<R: Read+Seek>(buffer: &mut R) -> Result<Header> {
        println!("Extracting header...");
        let mut magic = vec![0u8;8];
        buffer.read_exact(&mut magic)?;
        if magic != b"MsgStdBn" {
            return Err(Error::NotMSBT)
        }
        let endianness_raw = u16::read_from(buffer, bytestream::ByteOrder::BigEndian)?;
        let endianness;
        let endianness_bool;
        if endianness_raw == 0xFEFF{
            endianness = bytestream::ByteOrder::BigEndian;
            endianness_bool = true;
        } else {
            endianness = bytestream::ByteOrder::LittleEndian;
            endianness_bool = false;
        }
        buffer.seek(SeekFrom::Current(10))?;
        println!("Extracted header.");
        Ok(Header{
            magic: magic,
            endianness: endianness_bool,
            unk1: u16::read_from(buffer, endianness)?,
            unk2: u16::read_from(buffer, endianness)?,
            section_amount: u16::read_from(buffer, endianness)?,
            unk3: u16::read_from(buffer, endianness)?,
            filesize: u32::read_from(buffer, endianness)?,
        })
    }

    pub fn write_binary(section_amount: u16,section_sizes: u32, order: bytestream::ByteOrder) -> Result<Vec<u8>>{
        println!("Formatting header...");
        let mut result = Vec::<u8>::new();
        let section_size = 8 as u32;
        //binary tiem
        result.append(&mut b"MsgStdBn".to_vec());
        match order {
            ByteOrder::BigEndian => {
                result.append(&mut vec![0xFE, 0xFF]);
                result.append(&mut vec![0,0,1,3]);
                result.append(&mut section_amount.to_be_bytes().to_vec());
                result.append(&mut vec![0,0]);
                result.append(&mut (section_sizes+0x20).to_be_bytes().to_vec());
            }
            ByteOrder::LittleEndian => {
                result.append(&mut vec![0xFF, 0xFE]);
                result.append(&mut vec![0,0,1,3]);
                result.append(&mut section_amount.to_le_bytes().to_vec());
                result.append(&mut vec![0,0]);
                result.append(&mut (section_sizes+0x20).to_le_bytes().to_vec());
            }
        }
        let padding = 16 - result.len() %16;
        for _i in 0..padding{
            result.push(0x0);
        }
        println!("Formated header.");
        Ok(result)
    }
}