use std::{fs::File, io::Write};

use msbt::msbt;
use ::msbt::structs::{Header, ATR1, LBL1, TXT2};

fn main() -> ::msbt::Result<()> {
    let mut file = File::open("agb.msbt")?;
    let msbt = msbt::from_binary(&mut file)?;
    let strings = msbt::get_strings(msbt.clone())?;
    let new_msbt = msbt::to_binary(strings, msbt.endianness)?;
    let mut result = File::create("foo.msbt")?;
    result.write(&new_msbt)?;
    Ok(())
}
pub fn read_string(slice: &[u8], size: usize) -> Option<String> {
    assert!(2*size <= slice.len());
    let iter = (0..size)
        .map(|i| u16::from_be_bytes([slice[2*i+1], slice[2*i]]));

    std::char::decode_utf16(iter).collect::<Result<String, _>>().ok()
}