pub mod structs;
pub mod error;

use std::{fs::File, io::{Seek, SeekFrom}};

use structs::{header::Header, lbl1::LBL1};
use error::Result;

fn main() -> Result<()> {
    let mut file = File::open("agb.msbt")?;
    let header = Header::read_from(&mut file)?;
    file.seek(SeekFrom::Current(10))?;
    let lbl1 = LBL1::read_from(&mut file, header.endianness)?;
    print!("{:#?}", lbl1.labels);
    Ok(())
}
