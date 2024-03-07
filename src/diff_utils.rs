use crate::msbt::MSBTString;

pub fn get_added(original: Vec<MSBTString>, edited: Vec<MSBTString>) -> Vec<MSBTString> {
    let mut result = Vec::<MSBTString>::new();
    for string in edited {
        let index = original.iter().position(|s| s.label == string.label);
        if index.is_none(){
            result.push(string);
        }
    }
    return result;
}
pub fn get_deleted(original: Vec<MSBTString>, edited: Vec<MSBTString>) -> Vec<MSBTString> {
    let mut result = Vec::<MSBTString>::new();
    for string in original {
        let index = edited.iter().position(|s| s.label == string.label);
        if index.is_none(){
            result.push(string);
        }
    }
    return result;
}