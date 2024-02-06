use self::lbl1::{Label, LabelDef};

pub mod header;
pub mod lbl1;
pub mod atr1;
pub mod txt2;

#[derive(Debug, Clone)]
pub struct Header{
    magic: Vec<u8>,
    pub endianness: bool,
    unk1: u16,
    unk2: u16, //Version?
    pub section_amount: u16,
    unk3: u16,
    filesize: u32,
}

#[derive(Debug, Clone)]
pub struct LBL1{
    magic: Vec::<u8>,
    section_size: u32,
    block_amount: u32,
    pub offsets: Vec<LabelDef>,
    pub labels: Vec<Label>
}

#[derive(Debug, Clone)]
pub struct ATR1{ // Not enough data, since only Rhythm Heaven Megamix has been used for research.
    magic: Vec<u8>,
    section_size: u32,
    string_amount:u64
}

#[derive(Debug, Clone)]
pub struct TXT2{
    magic: Vec::<u8>,
    section_size: u32,
    string_amount: u32,
    pub offsets: Vec<u32>,
    pub strings: Vec<Vec<u8>>
}