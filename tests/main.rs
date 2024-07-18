use json_template::*;
use serde::{Deserialize, Serialize};
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
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file = manifest_dir.join("tests").join("data-from-file.json");
    let context = Context::new()
        .with_data(serde_json::json!({
            "data": {
                "time": "now"
            }
        }));
    let data: Data = Deserializer::new().deserialize_with_context(file, &context).expect("Failed to deserialize.");
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
    let deserializer = Deserializer::new();
    let context = Context::new()
        .with_directory(Some(directory.clone()))
        .with_data(serde_json::json!({
            "data": {
                "time": "now"
            }
        }));
    let data: Data = deserializer.deserialize_with_context(file, &context).expect("Failed to deserialize.");
    assert_eq!(data, Data {
        name: "Danilo".into(),
        age: 36,
        info: "Danilo is 36 years old.".into(),
        age_str: "36".into(),
        time: Some("now".into())
    })
}

#[test]
fn data_chain() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct MyNameIs {
        pub my: String,
        pub name: String,
        pub is: String
    }
    
    let file = include_str!("data-chain.json");
    let template = Deserializer::new();
    let data: MyNameIs = template.deserialize(file).expect("Failed to deserialize.");
    assert_eq!(data, MyNameIs {
        my: "Danilo".into(),
        name: "Danilo".into(),
        is: "Danilo".into()
    })
}

#[test]
fn custom_function() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Time {
        duration: std::time::Duration
    }

    let value = serde_json::json!({ "duration": "{time:5}" });
    let context = Context::new().with_function("time", |_deserializer, _context, placeholder| {
        let seconds = placeholder
            .path()
            .parse::<u64>()
            .map_err(|e| serde::de::Error::custom(e))?;
        let duration = std::time::Duration::from_secs(seconds);
        serde_json::to_value(&duration)
    });
    let data: Time = Deserializer::new().deserialize_with_context(value, &context).expect("Failed to deserialize");
    assert_eq!(data.duration, std::time::Duration::from_secs(5));
}