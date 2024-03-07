use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::File,
    io::{Read, Write},
    path::Path,
    string,
};

use ::msbt::msbt::MSBTString;
use bytestream::ByteOrder;
use clap::{Parser, ValueEnum};
use msbt::msbt;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command
    #[arg(value_enum)]
    action: Actions,

    /// File to extract, or to use as a base for diffing.
    original: String,

    /// Files to use for diffing.
    edited: Vec<String>,
}
#[derive(ValueEnum, Clone, Debug)]
enum Actions {
    EXTRACT,
    CREATE,
    DIFF,
    PATCH,
}

#[derive(Serialize, Deserialize, Clone)]
struct SerMsbt {
    is_big_endian: bool,
    has_attributes: bool,
    strings: HashMap<String, String>,
}

fn main() -> ::msbt::Result<()> {
    let args = Args::parse();
    match args.action {
        Actions::EXTRACT => return extract_msbt(args),
        Actions::CREATE => return create_msbt(args),
        Actions::DIFF => todo!(),
        Actions::PATCH => todo!(),
    }
}

fn extract_msbt(args: Args) -> ::msbt::Result<()> {
    let arg_filename = args.original.clone();
    let path = Path::new(&arg_filename);
    let extension = path.extension().unwrap().to_str().unwrap().to_lowercase();
    if extension == "msbt" {
        let filename = path.file_stem().unwrap().to_str().unwrap();
        let filepath = path.parent().unwrap();
        let mut file = File::open(args.original)?;

        let mut output_map = HashMap::new();
        let msbt = msbt::from_binary(&mut file)?;
        let strings = msbt::get_strings(msbt.clone())?;
        for string in strings {
            let mut parsed_string =
                ::msbt::structs::TXT2::parse_binary(string.string, msbt.endianness);
            parsed_string.truncate(parsed_string.len() - 1);
            output_map.insert(string.label, parsed_string);
        }
        let order = match msbt.endianness {
            bytestream::ByteOrder::BigEndian => true,
            bytestream::ByteOrder::LittleEndian => false,
        };
        let msbt_json = SerMsbt {
            is_big_endian: order,
            has_attributes: msbt.has_attributes,
            strings: output_map,
        };
        let serialized = toml::ser::to_string_pretty(&msbt_json).unwrap();
        let mut result = File::create(filepath.join(filename.to_owned() + ".toml"))?;
        result.write(serialized.as_bytes())?;
        Ok(())
    } else {
        Err(::msbt::Error::NotMSBT)
    }
}

fn create_msbt(args: Args) -> ::msbt::Result<()> {
    let arg_filename = args.original.clone();
    let path = Path::new(&arg_filename);
    let filename = path.file_stem().unwrap().to_str().unwrap();
    let filepath = path.parent().unwrap();
    let mut file = File::open(args.original)?;
    let toml = get_toml(file)?;
    let strings = get_strings_toml(&toml)?;
    let order = get_endianness_toml(&toml)?;
    let new_msbt = msbt::to_binary(strings, order)?;
    let mut result = File::create(filepath.join(filename.to_owned() + ".msbt"))?;
    result.write(&new_msbt)?;
    Ok(())
}

fn get_toml(mut file: File) -> ::msbt::Result<SerMsbt>{
    let mut toml_string = "".to_owned();
    let _ = file.read_to_string(&mut toml_string);
    Ok(toml::de::from_str(toml_string.as_str())?)
}

fn get_endianness_toml(toml: &SerMsbt) -> ::msbt::Result<bytestream::ByteOrder> {
    let mut strings = Vec::<MSBTString>::new();
    let mut i = 0;
    match toml.is_big_endian {
        true => Ok(bytestream::ByteOrder::BigEndian),
        false => Ok(bytestream::ByteOrder::LittleEndian),
    }
}

fn get_strings_toml(toml: &SerMsbt) -> ::msbt::Result<Vec<MSBTString>>{
    let mut strings = Vec::<MSBTString>::new();
    let mut i = 0;
    let order = match toml.is_big_endian {
        true => bytestream::ByteOrder::BigEndian,
        false => bytestream::ByteOrder::LittleEndian,
    };
    println!("Parsing {} string(s)...", toml.strings.len());
    for (label, string) in &toml.strings {
        let corrected_string = string.to_owned() + "\0";
        strings.push(MSBTString {
            index: i,
            label: label.to_string(),
            string: ::msbt::structs::TXT2::parse_string(&corrected_string, order).unwrap(),
        });
        i += 1;
    }
    println!("Parsed {} string(s).", strings.len());
    Ok(strings)
}
