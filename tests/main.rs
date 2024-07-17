use json_template::Template;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Data {
    name: String,
    age: usize,
    info: String,
    age_str: String,
    time: Option<String>
}

#[test]
fn from_file() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data-from-file.json");
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
    let mut template = Template::new().with_directory(Some(directory.clone()));
    template.set_data(serde_json::json!({
        "data": {
            "time": "now"
        },
    }));
    let data: Data = template.deserialize(path).expect("Failed to read file.");
    assert_eq!(data, Data {
        name: "Danilo".into(),
        age: 36,
        info: "Danilo is 36 years old.".into(),
        age_str: "36".into(),
        time: Some("now".into())
    })
}

#[test]
fn from_string() {
    let file = include_str!("data-from-code.json");
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
    let mut template = Template::new().with_directory(Some(directory.clone()));
    template.set_data(serde_json::json!({
        "data": {
            "time": "now"
        }
    }));
    let data: Data = template.deserialize(file).expect("Failed to deserialize.");
    assert_eq!(data, Data {
        name: "Danilo".into(),
        age: 36,
        info: "Danilo is 36 years old.".into(),
        age_str: "36".into(),
        time: Some("now".into())
    })
}
