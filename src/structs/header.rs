use std::io::Read;
use bytestream::StreamReader;
use crate::error::{Error, Result};

use super::Header;

impl Header{
    pub fn read_from<R: Read>(buffer: &mut R) -> Result<Header> {
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
}