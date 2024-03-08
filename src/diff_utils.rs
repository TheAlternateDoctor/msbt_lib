use std::{fs::File, io::{BufReader, Lines}};

use crate::msbt::MSBTString;

#[derive(Debug, Clone)]
pub struct StringDiff{
    state: State,
    label: String,
    string: String
}

#[derive(Debug, Clone, PartialEq)]
pub enum State{
    ADDED,
    DELETED,
    EDITED,
    NULL
}

pub fn get_added(original: Vec<MSBTString>, vec_edited: Vec<Vec<MSBTString>>) -> Vec<MSBTString> {
    let mut result = Vec::<MSBTString>::new();
    for edited in vec_edited{
        for string in edited {
            let index = original.iter().position(|s| s.label == string.label);
            if index.is_none(){
                result.push(string);
            }
        }
    }
    return result;
}

pub fn get_deleted(original: Vec<MSBTString>, vec_edited: Vec<Vec<MSBTString>>) -> Vec<MSBTString> {
    let mut result = Vec::<MSBTString>::new();
    for edited in vec_edited{
        for string in &original {
            let index = edited.iter().position(|s| s.label == string.label);
            if index.is_none(){
                result.push(string.to_owned());
            }
        }
    }
    return result;
}

pub fn get_edited(original: Vec<MSBTString>, vec_edited: Vec<Vec<MSBTString>>) -> Vec<MSBTString> {
    let mut result = Vec::<MSBTString>::new();
    for edited in vec_edited{
        for string in edited {
            let index = original.iter().position(|s| s.label == string.label);
            if !index.is_none(){
                let string_original = original.get(index.unwrap()).unwrap();
                if string_original.string != string.string{
                    result.push(string);
                }
            }
        }
    }
    return result;
}

pub fn convert_diff(diff: Lines<BufReader<File>>) -> ::msbt::Result<Vec<StringDiff>> {
    let mut result = Vec::<StringDiff>::new();
    let mut current_diff = StringDiff { state: State::NULL, label: "".to_owned(), string: "".to_owned() };
    for line in diff.flatten() {
        if line == "" {
            if current_diff.state != State::NULL{
                current_diff.string = current_diff.string.trim().to_owned();
                result.push(current_diff.clone());
                current_diff = StringDiff { state: State::NULL, label: "".to_owned(), string: "".to_owned() };
            }
        } else {
            if current_diff.state == State::NULL {
                let mut chars: Vec<char> = line.chars().collect();
                match *chars.first().unwrap(){
                    '+' => current_diff.state = State::ADDED,
                    '-' => current_diff.state = State::DELETED,
                    '~' => current_diff.state = State::EDITED,
                    _ => return Err(::msbt::Error::MalformedDiffUnrecognizedState)
                }
                chars.remove(0);
                current_diff.label = chars.into_iter().collect();
            } else {
                let mut edited_line = line.clone();
                edited_line.remove(0);
                edited_line.push('\n');
                current_diff.string.push_str(&edited_line);
            }
        }
    }
    Ok(result)
}

pub fn patch_diff(diff: Vec<StringDiff>, msbt: Vec<MSBTString>, order: bytestream::ByteOrder) -> ::msbt::Result<Vec<MSBTString>>{
    let mut new_msbt = msbt.clone();
    for string_diff in diff {
        let corrected_string = string_diff.string + "\0";
        match string_diff.state {
            State::ADDED => ::msbt::msbt::add_string(&mut new_msbt, string_diff.label, corrected_string, order),
            State::DELETED => ::msbt::msbt::delete_string_by_label(&mut new_msbt, string_diff.label),
            State::EDITED => ::msbt::msbt::edit_string_by_label(&mut new_msbt, string_diff.label, corrected_string, order),
            State::NULL => {},
        }
    }
    Ok(new_msbt)
}