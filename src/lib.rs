#![doc = include_str!("../README.md")]

#![deny(missing_docs)]
use std::path::Path;

use serde::de::DeserializeOwned;
use serde_json::*;

mod traits;
mod placeholder;
mod json;
mod context;

use traits::*;
use placeholder::*;
use json::*;
use context::*;

/// serde_value::from_str equivalent.
/// It will fail if the JSON contains any file placeholder because it doesn't know where to look for the files.
pub fn from_str<T: DeserializeOwned>(value: &str) -> Result<T> {
    from_str_with_dir(value, None)
}

/// Reads and deserializes a JSON file.
pub fn from_file<T: DeserializeOwned>(path: impl AsRef<std::path::Path>) -> Result<T> {
    let path = path.as_ref();
    let file = std::fs::read_to_string(path)
        .map_err(|e| serde::de::Error::custom(format!("{} - {}", e, path.display())))?;
    from_str_with_dir(&file, path.parent())
}

/// serde_value::from_str equivalent with a directory.
pub fn from_str_with_dir<T: DeserializeOwned>(value: &str, directory: Option<&Path>) -> Result<T> {
    let mut json = Json::from(serde_json::from_str::<Value>(&value)?);
    let mut context = Context::from(Value::from(json.clone()));
    context.set_directory(directory.map(|path| path.to_path_buf()));
    json.resolve_files(&context)?;
    let context = Context::from(Value::from(json.clone())); // TODO: Inefficient.
    json.resolve_placeholders(&context)?;
    Ok(serde_json::from_value(serde_json::Value::from(json))?)
}
