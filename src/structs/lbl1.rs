use std::io::{Read, Seek, SeekFrom};
use bytestream::StreamReader;
use crate::error::{Error, Result};

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
}