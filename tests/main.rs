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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Person {
    name: String,
    age: usize
}

#[test]
fn from_file() {
    let file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data-from-file.json");
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
    let file = include_str!("data-self-referencing.json");
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
fn placeholders_path() {
    let file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data-placeholders-path.json");
    let data: Person = Deserializer::new().deserialize(file).expect("Failed to deserialize.");
    assert_eq!(data, Person {
        name: "Danilo".into(),
        age: 36
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
    let data: MyNameIs = Deserializer::new().deserialize(file).expect("Failed to deserialize.");
    assert_eq!(data, MyNameIs {
        my: "Danilo".into(),
        name: "Danilo".into(),
        is: "Danilo".into()
    })
}

#[test]
fn path_segments() {
    let path = Path::new("data");
    let segments = path.segments();
    assert_eq!(segments.len(), 1);
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
            .str()
            .parse::<u64>()
            .map_err(|e| serde::de::Error::custom(e))?;
        let duration = std::time::Duration::from_secs(seconds);
        serde_json::to_value(&duration)
    });
    let data: Time = Deserializer::new().deserialize_with_context(value, &context).expect("Failed to deserialize");
    assert_eq!(data.duration, std::time::Duration::from_secs(5));
}

#[test]
fn placeholders() {
    let placeholder = "{time:5}";
    let placeholder = Placeholder::from_str(placeholder).expect("Failed to create placeholder.");
    assert_eq!(placeholder.value, "{time:5}");
    assert_eq!(placeholder.type_, Some("time".into()));

    let placeholder = "{{time:5}}";
    let placeholder = Placeholder::from_str(placeholder).expect("Failed to create placeholder.");
    assert_eq!(placeholder.value, "{{time:5}}");
    assert_eq!(placeholder.type_, None);

    let placeholders = "   {time}   {time:3}    ";
    let placeholders = Placeholder::placeholders(placeholders);
    assert_eq!(placeholders.len(), 2);
    assert_eq!(placeholders[0].value, "{time}");
    assert_eq!(placeholders[1].value, "{time:3}");

    let recursive_placeholders = "{time:{time:5}}  {time}";
    let recursive_placeholders = Placeholder::placeholders(recursive_placeholders);
    assert_eq!(recursive_placeholders.len(), 2);
    assert_eq!(recursive_placeholders[0].value, "{time:{time:5}}");
    assert_eq!(recursive_placeholders[1].value, "{time}");

    let placeholders_path = "{{file:data--self-referencing.json}.data}";
    let placeholders_path = Placeholder::placeholders(placeholders_path);
    assert_eq!(placeholders_path.len(), 1);
    assert_eq!(placeholders_path[0].value, "{{file:data--self-referencing.json}.data}");
}

#[test]
fn compose_function() {
    let file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data-composed-a-b.json");
    let data: Person = Deserializer::new().deserialize(file).expect("Failed to deserialize.");
    assert_eq!(data, Person {
        name: "Danilo".into(),
        age: 36
    })
}