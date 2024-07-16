# serde_json_placeholders

`serde_json_placeholders` is a library that allows you to use placeholders in your JSON files.

## Example

```rust
use serde_json_placeholders::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Data {
   name: String,
   age: usize,
   age_str: String,
   info: String
}

let json = r#"{
  "data": {
    "name": "Danilo",
    "age": 36
  },
  "name": "{data.name}",
  "age": "{data.age}",
  "age_str": "{string:data.age}",
  "info": "{data.name} is {data.age} years old."
}"#;
let data: Data = from_str(json).unwrap();
assert_eq!(data, Data {
   name: "Danilo".to_string(),
   age: 36,
   age_str: "36".to_string(),
   info: "Danilo is 36 years old.".to_string()
})
```

Note that the `age_str` field is a string, while the `age` field is a number.

Simple `"{age}"` placeholders are replaced with the corresponding value in the JSON object.

`"{string:age}"` placeholders are replaced with the corresponding value in the JSON object as a string.

Formatted placeholders like `"{data.name} is {data.age} years old."` are replaced with the corresponding values in the JSON object as strings.

Even though `"{string:data.name} is {string:data.age} years old."` would work, it is not necessary.

## Example with file references

```rust
use serde_json_placeholders::*;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Data {
   name: String,
   age: usize,
   age_str: String,
   info: String
}

let json = r#"{
  "data": "{file:tests/data.json}",
  "name": "{data.name}",
  "age": "{data.age}",
  "age_str": "{string:data.age}",
  "info": "{data.name} is {data.age} years old."
}"#;
let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
let directory = Some(directory.as_path());
let data: Data = from_str_with_dir(json, directory).unwrap();
assert_eq!(data, Data {
   name: "Danilo".to_string(),
   age: 36,
   age_str: "36".to_string(),
   info: "Danilo is 36 years old.".to_string()
})
```

or simply

```rust
use serde_json_placeholders::*;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Data {
   name: String,
   age: usize,
   age_str: String,
   info: String
}

let file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("tests")
    .join("file-reference.json");
let data: Data = from_file(file).unwrap();
assert_eq!(data, Data {
   name: "Danilo".to_string(),
   age: 36,
   age_str: "36".to_string(),
   info: "Danilo is 36 years old.".to_string()
})
```
