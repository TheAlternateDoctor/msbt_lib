use std::fs::File;

use msbt_lib::msbt;

fn main() -> msbt_lib::error::Result<()> {
    let mut file = File::open("agb.msbt")?;
    let strings = msbt::get_strings(&mut file)?;
    for msbt_string in strings{
        if msbt_string.index == 85{
            println!("({}){}: ", msbt_string.index, msbt_string.label);
            println!("{}",read_string(&msbt_string.string, msbt_string.string.len()/2).unwrap());
        }
    }
    Ok(())
}
pub fn read_string(slice: &[u8], size: usize) -> Option<String> {
    assert!(2*size <= slice.len());
    let iter = (0..size)
        .map(|i| u16::from_be_bytes([slice[2*i+1], slice[2*i]]));

    std::char::decode_utf16(iter).collect::<Result<String, _>>().ok()
}