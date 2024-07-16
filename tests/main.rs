use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Data {
    name: String,
    age: usize,
    info: String,
    age_str: String
}

#[test]
fn from_file() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/file-reference.json");
    let data: Data = serde_json_placeholders::from_file(path).unwrap();
    assert_eq!(data, Data {
        name: "Danilo".to_string(),
        age: 36,
        info: "Danilo is 36 years old.".to_string(),
        age_str: "36".to_string()
    })
}

#[test]
fn from_str_with_dir() {
    let file = include_str!("self-contained.json");
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
    let data: Data = serde_json_placeholders::from_str_with_dir(file, Some(directory.as_path())).unwrap();
    assert_eq!(data, Data {
        name: "Danilo".to_string(),
        age: 36,
        info: "Danilo is 36 years old.".to_string(),
        age_str: "36".to_string()
    })
}

#[test]
fn from_str() {
    let file = include_str!("self-contained.json");
    let data: Data = serde_json_placeholders::from_str(file).unwrap();
    assert_eq!(data, Data {
        name: "Danilo".to_string(),
        age: 36,
        info: "Danilo is 36 years old.".to_string(),
        age_str: "36".to_string()
    })
}
