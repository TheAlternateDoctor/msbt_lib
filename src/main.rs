use std::{fs::File, io::{Seek, SeekFrom}};

use bytestream::ByteOrder;
use msbt_lib::structs::header::Header;
use msbt_lib::structs::lbl1::LBL1;
use msbt_lib::error::{Error, Result};

fn main() -> Result<()> {
    let mut file = File::open("agb.msbt")?;
    let header = Header::read_from(&mut file)?;
    file.seek(SeekFrom::Current(10))?;
    let lbl1 = LBL1::read_from(&mut file, if header.endianness {ByteOrder::BigEndian} else {ByteOrder::LittleEndian})?;
    println!("{:#?}", lbl1.labels);
    Ok(())
}
