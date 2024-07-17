//! Template module.

use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{GetDot, Json, Placeholder, ToDeserializable};

/// A template.
#[derive(Default, Clone)]
pub struct Template {
    /// JSON data.
    pub data: Json,
    /// Directory.
    pub directory: Option<PathBuf>
}

impl Template {
    /// Create a new template.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set data.
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = data.into();
        self
    }

    /// Set data.
    pub fn set_data(&mut self, data: serde_json::Value) -> &mut Self {
        self.data = data.into();
        self
    }

    /// Override data.
    pub fn with_override(mut self, new_value: Value) -> Self {
        self.override_data(new_value);
        self
    }

    /// Override data.
    pub fn override_data(&mut self, new_value: Value) -> &mut Self {
        Json::override_value_recursive(&mut self.data.value, new_value);
        self
    }

    /// Add data.
    pub fn with_additional_data(mut self, new_value: Value) -> Self {
        self.add_data(new_value);
        self
    }

    /// Add data.
    pub fn add_data(&mut self, new_value: Value) -> &mut Self {
        Json::add_recursive(&mut self.data.value, new_value);
        self
    }

    /// Set directory.
    pub fn with_directory(mut self, directory: Option<PathBuf>) -> Self {
        self.directory = directory;
        self
    }

    /// Set directory.
    pub fn set_directory(&mut self, directory: Option<PathBuf>) -> &mut Self {
        self.directory = directory;
        self
    }

    /// Deserialize JSON String.
    pub fn deserialize<T: DeserializeOwned>(mut self, value: impl ToDeserializable) -> serde_json::Result<T> {
        let (directory, value) = value.to_deserializable()?;
        if let Some(directory) = directory {
            self.set_directory(Some(directory));
        }
        let value = serde_json::from_str::<Value>(&value)?;
        let mut value = Json::from(value);
        value.resolve_files(&self.clone())?;
        self.add_data(value.into());
        self.data.resolve_placeholders(&self.clone())?;
        Ok(serde_json::from_value(serde_json::Value::from(self.data))?)
    }
    
    /// Resolve the placeholder.
    pub fn resolve(&self, placeholder: &Placeholder) -> Option<Value> {
        let (is_file, is_str) = placeholder
            .type_
            .as_ref()
            .map(|type_| (type_ == "file", type_ == "string"))
            .unwrap_or((false, false));
        if is_file {
            self
            .directory
            .as_ref()
            .map(|directory| directory.join(placeholder.path()))
            .and_then(|path| Template::new().deserialize::<Value>(path).ok())
        } else {
            self.data.value.get_dot(placeholder.path()).cloned().map(|value| {
                if is_str {
                    Value::String(Json::from(value).to_string())
                } else {
                    value
                }
            })    
        }
    }
}

impl From<serde_json::Value> for Template {
    fn from(data: serde_json::Value) -> Self {
        let data = Json::from(data);
        let directory = None;
        Self { data, directory }
    }
}