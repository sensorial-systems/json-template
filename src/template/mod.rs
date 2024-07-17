//! Template module.

use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{Functions, GetDot, Json, Placeholder, ToDeserializable};

/// A template.
#[derive(Clone)]
pub struct Template {
    /// JSON data.
    pub data: Json,
    /// Directory.
    pub directory: Option<PathBuf>,
    /// Functions.
    pub functions: Functions
}

impl Default for Template {
    fn default() -> Self {
        let data = Default::default();
        let directory = Default::default();
        let functions = Default::default();
        Self { data, directory, functions }        
    }
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
        value.resolve_files(&self.clone())?; // TODO: Why do we have to resolve_files before add_data? Answer: Because unresolved placeholders will be overridden by add_data
        self.add_data(value.into());
        self.data.resolve_placeholders(&self.clone())?; // TODO: Unify resolve_placeholders with resolve_files by making a chain resolver (allow placeholder to point to a placeholder)
        Ok(serde_json::from_value(serde_json::Value::from(self.data))?)
    }
    
    /// Resolve the placeholder.
    pub fn resolve(&self, placeholder: &Placeholder) -> Option<Value> {
        if let Some(type_) = placeholder.type_.as_ref() {
            self
                .functions
                .get(type_)
                .and_then(|function| function(self, placeholder))
        } else {
            self.data.value.get_dot(placeholder.path()).cloned()
        }
    }
}

impl From<serde_json::Value> for Template {
    fn from(data: serde_json::Value) -> Self {
        Template::new().with_data(data)
    }
}
