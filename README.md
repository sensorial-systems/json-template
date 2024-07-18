[![](https://dcbadge.vercel.app/api/server/rzaesS82MT)](https://discord.gg/rzaesS82MT)

# json-template

`json-template` is a library that allows you to create JSON templates.

[file.json]
```json
{
   "personal_info": "{file:additional.json}",
   "info": "{personal_info.name} {personal_info.last_name} is {personal_info.age} years old"
}
```

[additional.json]
```json
{
   "name": "Danilo",
   "last_name": "Guanabara",
   "age": 36
}
```

Can be rendered to
```json
{
   "personal_info": {
      "age": 36,
      "name": "Danilo",
      "last_name": "Guanabara"
   },
   "info": "Danilo Guanabara is 36 years old"
}
```

## Functionalities

### Chained resolver
```json
{
   "my": "Danilo",
   "name": "{my}",
   "is": "{name}
}
```

### Functions

#### Built-in functions

| Function | Description |
|----------|-------------|
| `{file:path}`|Loads a file from a relative path. Its base directory is automatically set if you deserialize a file. You can also set it manually using `Context::set_directory`.|
|`{string:path}`| Transforms a `serde_json::Value` to `serde_json::Value::String`. It's useful if you need to deserialize a Number as a String.

Check `Custom Functions` code example to learn how to create a custom function.

## Code examples

### From memory

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
let context = Context::new()
   .with_data(serde_json::json!({
      "data": {
         "time": "now"
      }
   }));
let data: Data = Deserializer::new().deserialize_with_context(json, &context).unwrap();
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

### From file

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
let context = Context::new()
   .with_data(serde_json::json!({
      "data": {
         "time": "now"
      }
   }))
   .with_directory(Some(directory));

let data: Data = Deserializer::new().deserialize_with_context(json, &context).unwrap();

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
let context = Context::new()
   .with_data(serde_json::json!({
      "data": {
         "time": "now"
      }
   }));

let data: Data = Deserializer::new().deserialize_with_context(file, &context).unwrap();

assert_eq!(data, Data {
   name: "Danilo".into(),
   age: 36,
   age_str: "36".into(),
   info: "Danilo is 36 years old.".into(),
   time: "now".into()
})
```

### Custom functions

```rust
use json_template::*;
use serde::{Serialize, Deserialize};

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
```