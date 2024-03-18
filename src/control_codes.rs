use std::collections::VecDeque;

use bytestream::ByteOrder;

use crate::structs::txt2::{convert_char, read_char};

#[derive(Debug)]
pub struct ControlCode{
    tag_group: u16,
    tag_type: u16,
    params_size: u16,
    params: Vec<u8>
}

pub fn convert_control_code_binary(string: &mut VecDeque<u8>, order: bytestream::ByteOrder) -> String{

    let mut control_code = ControlCode {tag_group:0,tag_type:0,params_size:0,params:Vec::<u8>::new()};

    //Reading the group of the code
    let byte_temp = [string.pop_front().unwrap(), string.pop_front().unwrap()];
    let byte = read_char(byte_temp, order);
    control_code.tag_group = byte;

    //Reading the type of the code
    let byte_temp = [string.pop_front().unwrap(), string.pop_front().unwrap()];
    let byte = read_char(byte_temp, order);
    control_code.tag_type = byte;

    //Reading the amount of arguments
    let byte_temp = [string.pop_front().unwrap(), string.pop_front().unwrap()];
    let byte = read_char(byte_temp, order);
    control_code.params_size = byte;

    //Reading the arguments
    for _i in 0..control_code.params_size {
        control_code.params.push(string.pop_front().unwrap());
    }
    // Now we write the final string
    let mut control_string = String::from("[RawCmd ");
    control_string += &control_code.tag_group.to_string();
    control_string += ".";
    control_string += &control_code.tag_type.to_string();
    if control_code.params_size > 0{
        control_string += " ";
        for code in control_code.params{
            control_string += &format!("{code:02X}");
            control_string += "_";
        }
        control_string.truncate(control_string.len()-1);
    }
    control_string += "]";
    control_string
}

pub fn convert_control_code_close_binary(string: &mut VecDeque<u8>, order: bytestream::ByteOrder) -> String {

    let mut control_code = ControlCode {tag_group:0,tag_type:0,params_size:0,params:Vec::<u8>::new()};

    //Reading the group of the code
    let byte_temp = [string.pop_front().unwrap(), string.pop_front().unwrap()];
    let byte = read_char(byte_temp, order);
    control_code.tag_group = byte;

    //Reading the type of the code
    let byte_temp = [string.pop_front().unwrap(), string.pop_front().unwrap()];
    let byte = read_char(byte_temp, order);
    control_code.tag_type = byte;
    
    // Now we write the final string
    let mut control_string = String::from("[/RawCmd ");
    control_string += &control_code.tag_group.to_string();
    control_string += ".";
    control_string += &control_code.tag_type.to_string();
    control_string += "]";

    control_string
}

pub fn convert_control_code(code: &str, order: bytestream::ByteOrder) -> Vec<u8>{
    let mut raw_bytes = Vec::<u8>::new();
    let mut bare_code = code.to_string();
    bare_code.remove(0);
    bare_code.pop();
    let bare_content = bare_code.split(' ').collect::<Vec<&str>>();
    match bare_content[0] {
        "RawCmd" =>{
            let mut control_code = ControlCode{ 
                tag_group: 0, 
                tag_type: 0, 
                params_size: 0, 
                params: Vec::<u8>::new() 
            };
            let code_def = bare_content[1].split('.').collect::<Vec<&str>>();
            control_code.tag_group = code_def[0].parse().unwrap();
            control_code.tag_type = code_def[1].parse().unwrap();

            if bare_content.len() > 2{
                control_code.params_size = ((bare_content[2].len()+1)/3) as u16;
                for byte in bare_content[2].split('_').collect::<Vec<&str>>() {
                    control_code.params.push(u8::from_str_radix(byte, 16).unwrap());
                }
            } else {
                control_code.params_size = 0u16;
            }
            raw_bytes.push(0x0E);
            raw_bytes.push(0x00);
            match order{
                ByteOrder::BigEndian => {
                    raw_bytes.append(&mut control_code.tag_group.to_be_bytes().to_vec());
                    raw_bytes.append(&mut control_code.tag_type.to_be_bytes().to_vec());
                    raw_bytes.append(&mut control_code.params_size.to_be_bytes().to_vec());
                },
                ByteOrder::LittleEndian => {
                    raw_bytes.append(&mut control_code.tag_group.to_le_bytes().to_vec());
                    raw_bytes.append(&mut control_code.tag_type.to_le_bytes().to_vec());
                    raw_bytes.append(&mut control_code.params_size.to_le_bytes().to_vec());
                },
            }
            if control_code.params_size != 0{
                raw_bytes.append(&mut control_code.params);
            }
        }
        _ => {
            for char in code.chars(){
                raw_bytes.append(&mut convert_char(char, order));
            }
        }
    }
    raw_bytes
}

pub fn convert_control_code_close(code: &str, order: bytestream::ByteOrder) -> Vec<u8>{
    let mut raw_bytes = Vec::<u8>::new();
    let mut bare_code = code.to_string();
    bare_code.remove(0);
    bare_code.remove(0);
    bare_code.pop();
    let bare_content = bare_code.split(' ').collect::<Vec<&str>>();
    match bare_content[0]{
        "RawCmd" => {
            let code_def = bare_content[1].split('.').collect::<Vec<&str>>();
            let tag_group: u16 = code_def[0].parse().unwrap();
            let tag_type: u16 = code_def[1].parse().unwrap();

            raw_bytes.push(0x0F);
            raw_bytes.push(0x00);
            match order{
                ByteOrder::BigEndian => {
                    raw_bytes.append(&mut tag_group.to_be_bytes().to_vec());
                    raw_bytes.append(&mut tag_type.to_be_bytes().to_vec());
                },
                ByteOrder::LittleEndian => {
                    raw_bytes.append(&mut tag_group.to_le_bytes().to_vec());
                    raw_bytes.append(&mut tag_type.to_le_bytes().to_vec());
                },
            }
        }
        _=> {
            for char in code.chars(){
                raw_bytes.append(&mut convert_char(char, order));
            }
        }
    }
    raw_bytes
}