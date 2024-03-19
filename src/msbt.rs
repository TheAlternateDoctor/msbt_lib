use std::io::{Read, Seek};

use bytestream::ByteOrder;

use crate::error::Result;
use crate::structs::{Header, ATR1, LBL1, TXT2};

#[derive(Clone)]
pub struct MSBT {
    _header: Header,
    lbl1: LBL1,
    _atr1: ATR1,
    txt2: TXT2,
    pub endianness: bytestream::ByteOrder,
    pub has_attributes: bool,
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct MSBTString {
    pub index: u32,
    pub label: String,
    pub string: Vec<u8>,
}

pub fn from_binary<R: Read + Seek>(buffer: &mut R) -> Result<MSBT> {
    let header = Header::read_from(buffer)?;
    let byte_order = if header.endianness {
        ByteOrder::BigEndian
    } else {
        ByteOrder::LittleEndian
    };
    let lbl1 = LBL1::read_from(buffer, byte_order)?;
    let atr1 = ATR1::read_from(buffer, byte_order)?;
    let mut has_attributes = true;
    if atr1.section_size == 0 {
        has_attributes = false;
    }
    let txt2 = TXT2::read_from(buffer, byte_order)?;
    Ok(MSBT {
        _header: header,
        lbl1,
        _atr1: atr1,
        txt2,
        endianness: byte_order,
        has_attributes,
    })
}

pub fn get_strings(msbt: MSBT) -> Result<Vec<MSBTString>> {
    let mut msbt_strings = Vec::<MSBTString>::new();
    for label in msbt.lbl1.labels {
        let string = MSBTString {
            index: label.string_index,
            label: label.label,
            string: msbt.txt2.strings[(label.string_index) as usize].clone(),
        };
        msbt_strings.push(string);
    }
    Ok(msbt_strings)
}

pub fn add_string_raw(msbt_strings: &mut Vec<MSBTString>, label: String, string: Vec<u8>) {
    let last = msbt_strings.iter().map(|c| c.index).max().unwrap();
    let new_string = MSBTString {
        index: last + 1,
        label,
        string,
    };
    msbt_strings.push(new_string);
}

pub fn add_string(
    msbt_strings: &mut Vec<MSBTString>,
    label: String,
    string: String,
    order: bytestream::ByteOrder,
) {
    let last = msbt_strings.iter().map(|c| c.index).max().unwrap();
    let new_string: Vec<u8> = match order {
        ByteOrder::BigEndian => string
            .encode_utf16()
            .flat_map(|c| c.to_be_bytes())
            .collect(),
        ByteOrder::LittleEndian => string
            .encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect(),
    };
    let new_msbt_string = MSBTString {
        index: last + 1,
        label,
        string: new_string,
    };
    msbt_strings.push(new_msbt_string);
}

pub fn delete_string_by_index(msbt_strings: &mut Vec<MSBTString>, index: u32) {
    let vec_index = msbt_strings.iter().position(|s| s.index == index).unwrap();
    delete_string(msbt_strings, vec_index);
}

pub fn delete_string_by_label(msbt_strings: &mut Vec<MSBTString>, label: String) {
    let vec_index = msbt_strings.iter().position(|s| s.label == label).unwrap();
    delete_string(msbt_strings, vec_index);
}
fn delete_string(msbt_strings: &mut Vec<MSBTString>, vec_index: usize) {
    let msbt_index = msbt_strings[vec_index].index;
    msbt_strings.remove(vec_index);
    for string in msbt_strings {
        if string.index > msbt_index {
            string.index -= 1;
        }
    }
}

pub fn edit_string_by_label(
    msbt_strings: &mut [MSBTString],
    label: String,
    string: String,
    order: bytestream::ByteOrder,
) {
    let new_string: Vec<u8> = match order {
        ByteOrder::BigEndian => string
            .encode_utf16()
            .flat_map(|c| c.to_be_bytes())
            .collect(),
        ByteOrder::LittleEndian => string
            .encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect(),
    };
    match msbt_strings.iter().position(|s| s.label == label) {
        Some(index) => {
            let old_index = msbt_strings.get(index).unwrap().index;
            msbt_strings[index] = MSBTString {
                index: old_index,
                label,
                string: new_string,
            };
        }
        None => println!("No label named \"{}\" found!", label),
    };
}

pub fn to_binary(msbt_strings: Vec<MSBTString>, order: bytestream::ByteOrder) -> Result<Vec<u8>> {
    println!(
        "Formatting {} strings to MSBT, {} endian",
        msbt_strings.len(),
        match order {
            ByteOrder::BigEndian => "big",
            ByteOrder::LittleEndian => "little",
        }
    );
    let mut file = Vec::<u8>::new();
    let mut lbl1 = LBL1::write_binary(msbt_strings.clone(), order)?;
    let mut atr1 = ATR1::write_binary(msbt_strings.clone(), order)?;
    let mut txt2 = TXT2::write_binary(msbt_strings.clone(), order)?;
    let mut header = Header::write_binary(3, (lbl1.len() + atr1.len() + txt2.len()) as u32, order)?;
    file.append(&mut header);
    file.append(&mut lbl1);
    file.append(&mut atr1);
    file.append(&mut txt2);
    println!("Formatted MSBT.");
    Ok(file)
}
