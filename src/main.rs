use std::{fs::File, io::Write};

use msbt::msbt;
use ::msbt::structs::TXT2;

fn main() -> ::msbt::Result<()> {
    let mut file = File::open("agb.msbt")?;
    let msbt = msbt::from_binary(&mut file)?;
    let strings = msbt::get_strings(msbt.clone())?;
    let txt2 = TXT2::write_binary(strings, msbt.endianness)?;
    // let mut i = 0;
    // for byte in &txt2{
    //         print!("{:#x} ", byte);
    //         i+=1;
    //         if i > 15{
    //             i = 0;
    //             print!("\n");
    //     }
    // }
    let mut result = File::create("foo.txt")?;
    result.write(&txt2)?;
    Ok(())
}
pub fn read_string(slice: &[u8], size: usize) -> Option<String> {
    assert!(2*size <= slice.len());
    let iter = (0..size)
        .map(|i| u16::from_be_bytes([slice[2*i+1], slice[2*i]]));

    std::char::decode_utf16(iter).collect::<Result<String, _>>().ok()
}