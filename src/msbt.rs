use std::io::{Read, Seek};

use bytestream::ByteOrder;

use crate::structs::{Header, ATR1, LBL1, TXT2};
use crate::error::{Error, Result};


#[derive(Debug, Clone)]
pub struct MSBTString {
    pub index: u32,
    pub label: String,
    pub string: Vec<u8>
}

pub fn get_strings<R: Read+Seek>(buffer: &mut R) -> Result<Vec<MSBTString>> {
    let header = Header::read_from(buffer)?;
    let byte_order = if header.endianness {ByteOrder::BigEndian} else {ByteOrder::LittleEndian};
    let lbl1 = LBL1::read_from(buffer, byte_order)?;
    let atr1 = ATR1::read_from(buffer, byte_order)?;
    let txt2 = TXT2::read_from(buffer, byte_order)?;
    let mut msbt_strings = Vec::<MSBTString>::new();
    for label in lbl1.labels{
        let string = MSBTString{
            index: label.string_index,
            label: label.label,
            string: txt2.strings[(label.string_index) as usize].clone(),
        };
        msbt_strings.push(string);
    }
    Ok(msbt_strings)
}