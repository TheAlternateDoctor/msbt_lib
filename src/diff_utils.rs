use crate::msbt::MSBTString;

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