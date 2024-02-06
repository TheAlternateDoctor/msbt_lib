use std::{fs::File, io::{Seek, SeekFrom}};

use bytestream::ByteOrder;
use msbt_lib::structs::{ATR1, Header, LBL1, TXT2};
use msbt_lib::error::Result;

fn main() -> Result<()> {
    let mut file = File::open("agb.msbt")?;
    let header = Header::read_from(&mut file)?;
    file.seek(SeekFrom::Current(10))?;
    let lbl1 = LBL1::read_from(&mut file, if header.endianness {ByteOrder::BigEndian} else {ByteOrder::LittleEndian})?;
    let atr1 = ATR1::read_from(&mut file, if header.endianness {ByteOrder::BigEndian} else {ByteOrder::LittleEndian})?;
    let txt2 = TXT2::read_from(&mut file, if header.endianness {ByteOrder::BigEndian} else {ByteOrder::LittleEndian})?;
    println!("{:#x?}", txt2.strings);
    Ok(())
}
