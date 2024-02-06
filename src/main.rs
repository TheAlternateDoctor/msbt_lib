use std::fs::File;

use msbt::msbt;

fn main() -> ::msbt::Result<()> {
    let mut file = File::open("agb.msbt")?;
    let msbt = msbt::from_binary(&mut file)?;
    let mut strings = msbt::get_strings(msbt)?;
    let mut vec_index = strings.iter().position(|s| s.label == "agbHoppingL_tutorial_b_01").unwrap();
    println!("({}):{}", strings[vec_index].index, strings[vec_index].label);
    msbt::delete_string_by_label(&mut strings, "agbHair_tutorial_e_02".to_owned());
    vec_index = strings.iter().position(|s| s.label == "agbHoppingL_tutorial_b_01").unwrap();
    println!("({}):{}", strings[vec_index].index, strings[vec_index].label);
    Ok(())
}
pub fn read_string(slice: &[u8], size: usize) -> Option<String> {
    assert!(2*size <= slice.len());
    let iter = (0..size)
        .map(|i| u16::from_be_bytes([slice[2*i+1], slice[2*i]]));

    std::char::decode_utf16(iter).collect::<Result<String, _>>().ok()
}