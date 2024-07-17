[![](https://dcbadge.vercel.app/api/server/rzaesS82MT)](https://discord.gg/rzaesS82MT)

# json-template

`json-template` is a library that allows you to use placeholders in your JSON files.

## Example

```rust
use json_template::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Data {
   name: String,
   age: usize,
   age_str: String,
   info: String,
   time: String
}

let json = r#"{
  "data": {
    "name": "Danilo",
    "age": 36
  },
  "name": "{data.name}",
  "age": "{data.age}",
  "age_str": "{string:data.age}",
  "info": "{data.name} is {data.age} years old.",
  "time": "{data.time}"
}"#;
let data: Data = Template::new()
   .with_data(serde_json::json!({
      "data": {
         "time": "now"
      }
   }))
   .deserialize(json).unwrap();
assert_eq!(data, Data {
   name: "Danilo".into(),
   age: 36,
   age_str: "36".into(),
   info: "Danilo is 36 years old.".into(),
   time: "now".into()
})
```

Note that the `age_str` field is a string, while the `age` field is a number.

Simple `"{age}"` placeholders are replaced with the corresponding value in the JSON object.

`"{string:age}"` placeholders are replaced with the corresponding value in the JSON object as a string.

Formatted placeholders like `"{data.name} is {data.age} years old."` are replaced with the corresponding values in the JSON object as strings.

Even though `"{string:data.name} is {string:data.age} years old."` would work, it is not necessary.

## Example with file references

```rust
use json_template::*;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Data {
   name: String,
   age: usize,
   age_str: String,
   info: String,
   time: String
}

let json = r#"{
  "data": "{file:tests/data.json}",
  "name": "{data.name}",
  "age": "{data.age}",
  "age_str": "{string:data.age}",
  "info": "{data.name} is {data.age} years old.",
  "time": "{data.time}"
}"#;

let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
let template = Template::new()
   .with_data(serde_json::json!({
      "data": {
         "time": "now"
      }
   }))
   .with_directory(Some(directory));

let data: Data = template.deserialize(json).unwrap();

assert_eq!(data, Data {
   name: "Danilo".into(),
   age: 36,
   age_str: "36".into(),
   info: "Danilo is 36 years old.".into(),
   time: "now".into()
})
```

or simply

```rust
use json_template::*;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Data {
   name: String,
   age: usize,
   age_str: String,
   info: String,
   time: String
}

let file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("tests")
    .join("data-from-file.json");

let data: Data = Template::new()
   .with_data(serde_json::json!({
      "data": {
         "time": "now"
      }
   }))
   .deserialize(file).unwrap();

assert_eq!(data, Data {
   name: "Danilo".into(),
   age: 36,
   age_str: "36".into(),
   info: "Danilo is 36 years old.".into(),
   time: "now".into()
})
```
