//! Context module.

use std::path::PathBuf;

use serde_json::Value;

use crate::{GetDot, Json, Placeholder};

/// A context.
pub struct Context {
    /// JSON data.
    pub data: serde_json::Value,
    /// Directory.
    pub directory: Option<PathBuf>
}

impl Context {
    /// Create a new context.
    pub fn resolve(&self, placeholder: &Placeholder) -> Option<Value> {
        if placeholder.is_file {
            self
            .directory
            .as_ref()
            .map(|directory| directory.join(placeholder.path()))
            .and_then(|path| crate::from_file::<Value>(path).ok())
        } else {
            self.data.get_dot(placeholder.path()).cloned().map(|value| {
                if placeholder.is_str {
                    Value::String(Json::from(value).to_string())
                } else {
                    value
                }
            })    
        }
    }

    /// Set directory.
    pub fn set_directory(&mut self, directory: Option<PathBuf>) {
        self.directory = directory;
    }
}

impl From<serde_json::Value> for Context {
    fn from(data: serde_json::Value) -> Self {
        let directory = None;
        Self { data, directory }
    }
}