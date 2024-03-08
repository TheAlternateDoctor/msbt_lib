use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, Read, Write};
use std::path::Path;

use diff_utils::convert_diff;
use ::msbt::msbt::MSBTString;
use clap::{Parser, ValueEnum};
use msbt::msbt;
use serde::{Deserialize, Serialize};

mod diff_utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    action: Actions,

    /// File to extract, or to use as a base for diffing.
    original: String,

    /// Files to use for diffing, or diff files to apply.
    edited: Vec<String>,
}
#[derive(ValueEnum, Clone, Debug)]
enum Actions {
    /// Converts an MSBT to TOML.
    Extract,
    /// Converts a TOML to MSBT.
    Create,
    /// Creates a diff file between <ORIGINAL> and all the files in [EDITED].
    Diff,
    /// Applies a diff file to an MSBT or TOML file.
    Patch,
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
        Actions::Extract => extract_msbt(args),
        Actions::Create => create_msbt_args(args),
        Actions::Diff => diff_msbt(args),
        Actions::Patch => patch_msbt(args),
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
        result.write_all(serialized.as_bytes())?;
        Ok(())
    } else {
        Err(::msbt::Error::NotMSBT)
    }
}

fn create_msbt_args(args: Args) -> ::msbt::Result<()> {
    let arg_filename = args.original.clone();
    let path = Path::new(&arg_filename);
    let filename = path.file_stem().unwrap().to_str().unwrap();
    let filepath = path.parent().unwrap();
    let file = File::open(args.original)?;
    let toml = get_toml(file)?;
    let strings = get_strings_toml(&toml)?;
    let order = get_endianness_toml(&toml)?;
    let _ = create_msbt(filepath.join(filename.to_owned() + ".msbt").into_os_string().into_string().unwrap(), strings, order);
    Ok(())
}

fn create_msbt(filename: String, msbt: Vec<MSBTString>, order: bytestream::ByteOrder) -> ::msbt::Result<()>{
    let new_msbt = msbt::to_binary(msbt, order)?;
    let mut result = File::create(filename)?;
    result.write_all(&new_msbt)?;
    Ok(())
}

// fn create_toml(filename: String, msbt: Vec<MSBTString>, order: bytestream::ByteOrder) -> ::msbt::Result<()>{

// }

fn diff_msbt(args: Args) -> ::msbt::Result<()> {
    let arg_filename = args.original.clone();
    let path = Path::new(&arg_filename);
    let filename = path.file_stem().unwrap().to_str().unwrap();
    let filepath = path.parent().unwrap();
    let extension = path.extension().unwrap().to_str().unwrap().to_lowercase();

    //Getting original strings....
    let orig_strings;
    let hash;
    let endianness;
    if extension == "msbt" {
        let bytes = fs::read(args.original.clone()).unwrap();
        hash = sha256::digest(bytes);
        let mut file = File::open(args.original)?;
        let msbt = msbt::from_binary(&mut file)?;
        endianness = msbt.endianness;
        orig_strings = msbt::get_strings(msbt.clone())?;
    } else { //Just assume it's toml
        let file = File::open(args.original)?;
        let toml = get_toml(file)?;
        endianness = get_endianness_toml(&toml)?;
        orig_strings = get_strings_toml(&toml)?;
        hash = "".to_owned();
    }

    //Getting edited strings...
    let mut edited_strings = Vec::<Vec<MSBTString>>::new();
    for path_edited in args.edited {
        let arg_filename = path_edited.clone();
        let path = Path::new(&arg_filename);
        let extension = path.extension().unwrap().to_str().unwrap().to_lowercase();
        let edited_string_single = if extension == "msbt" {
            let mut file = File::open(path_edited)?;
            let msbt = msbt::from_binary(&mut file)?;
            msbt::get_strings(msbt.clone())?
        } else { //Just assume it's toml
            let file = File::open(path_edited)?;
            let toml = get_toml(file)?;
            get_strings_toml(&toml)?
        };
        edited_strings.push(edited_string_single);
    }
    let added_strings = diff_utils::get_added(orig_strings.clone(), edited_strings.clone());
    let deleted_strings = diff_utils::get_deleted(orig_strings.clone(), edited_strings.clone());
    let edited_strings = diff_utils::get_edited(orig_strings, edited_strings);

    let mut diff_file = File::create(filepath.join(filename.to_owned()+".msbd.txt"))?;

    //Writing file
    let _ = diff_file.write((filename.to_owned()+"\n").as_bytes());
    let _ = diff_file.write((filename.to_owned()+"\n").as_bytes());
    if !hash.is_empty(){
        let _ = diff_file.write((hash+"\n").as_bytes());
    }
    let _ = diff_file.write("\n".as_bytes());

    //Writing added strings...
    for string in added_strings{
        let label = "+".to_owned()+&string.label+"\n";
        let _ = diff_file.write(label.as_bytes());
        let mut parsed_string = ::msbt::structs::TXT2::parse_binary(string.string, endianness);
        parsed_string.truncate(parsed_string.len() - 1);
        parsed_string = parsed_string.replace('\n', "\n>");
        let _ = diff_file.write((">".to_owned()+&parsed_string+"\n").as_bytes());
        let _ = diff_file.write("\n".as_bytes());
    }

    //Writing deleted strings...
    for string in deleted_strings{
        let label = "-".to_owned()+&string.label+"\n";
        let _ = diff_file.write(label.as_bytes());
        let _ = diff_file.write("\n".as_bytes());
    }

    //Writing edits...
    for string in edited_strings{
        let label = "~".to_owned()+&string.label+"\n";
        let _ = diff_file.write(label.as_bytes());
        let mut parsed_string = ::msbt::structs::TXT2::parse_binary(string.string, endianness);
        parsed_string.truncate(parsed_string.len() - 1);
        parsed_string = parsed_string.replace('\n', "\n>");
        let _ = diff_file.write((">".to_owned()+&parsed_string+"\n").as_bytes());
        let _ = diff_file.write("\n".as_bytes());
    }
    Ok(())
}

fn patch_msbt(args: Args) -> ::msbt::Result<()> {
    let diff_file = File::open(args.edited.first().unwrap())?;
    let mut lines = io::BufReader::new(diff_file).lines();
    let _final_filename = lines.next().unwrap()?;
    let patch_name = lines.next().unwrap()?;
    let sha256 = lines.next().unwrap()?;
    if sha256 != "\n" {
        let bytes = fs::read(args.original.clone()).unwrap();
        let hash = sha256::digest(bytes);
        if hash != sha256 {
            return Err(::msbt::Error::BadHash);
        }
        lines.next();
    }

    let arg_filename = args.original.clone();
    let path = Path::new(&arg_filename);
    let extension = path.extension().unwrap().to_str().unwrap().to_lowercase();
    if extension == "msbt" {
        let _filename = path.file_stem().unwrap().to_str().unwrap();
        let filepath = path.parent().unwrap();
        let mut file = File::open(args.original)?;

        let msbt = msbt::from_binary(&mut file)?;
        let strings = msbt::get_strings(msbt.clone())?;
        
        let diff = convert_diff(lines).unwrap();
        let new_msbt = diff_utils::patch_diff(diff, strings, msbt.endianness)?;
        match create_msbt(filepath.join(patch_name.to_owned() + ".msbt").into_os_string().into_string().unwrap(), new_msbt, msbt.endianness){
            Ok(_) => {},
            Err(_) => panic!("Error writing MSBT file."),
        };
    }
    Ok(())
}

fn get_toml(mut file: File) -> ::msbt::Result<SerMsbt>{
    let mut toml_string = "".to_owned();
    let _ = file.read_to_string(&mut toml_string);
    Ok(toml::de::from_str(toml_string.as_str())?)
}

fn get_endianness_toml(toml: &SerMsbt) -> ::msbt::Result<bytestream::ByteOrder> {
    match toml.is_big_endian {
        true => Ok(bytestream::ByteOrder::BigEndian),
        false => Ok(bytestream::ByteOrder::LittleEndian),
    }
}

fn get_strings_toml(toml: &SerMsbt) -> ::msbt::Result<Vec<MSBTString>>{
    let mut strings = Vec::<MSBTString>::new();
    let order = match toml.is_big_endian {
        true => bytestream::ByteOrder::BigEndian,
        false => bytestream::ByteOrder::LittleEndian,
    };
    println!("Parsing {} string(s)...", toml.strings.len());
    for (i, (label, string)) in toml.strings.iter().enumerate() {
        let corrected_string = string.to_owned() + "\0";
        strings.push(MSBTString {
            index: i as u32,
            label: label.to_string(),
            string: ::msbt::structs::TXT2::parse_string(&corrected_string, order).unwrap(),
        });
    }
    println!("Parsed {} string(s).", strings.len());
    Ok(strings)
}
