//! Context module.

use std::path::PathBuf;

use serde_json::Value;

use crate::{Deserializer, Functions, GetDot, Placeholder, JSON};

/// Deserialization context.
#[derive(Default, Clone)]
pub struct Context {
    /// JSON data.
    pub data: Value,
    /// Directory.
    pub directory: Option<PathBuf>,
    /// Functions.
    pub functions: Functions,
    /// JSON data being resolved.
    current: Value
}

impl Context {
     /// Create a new template.
     pub fn new() -> Self {
        Self::default()
    }

    /// Set data.
    pub fn with_data(mut self, data: Value) -> Self {
        self.data = data.into();
        self
    }

    /// Set data.
    pub fn set_data(&mut self, data: Value) -> &mut Self {
        self.data = data.into();
        self
    }

    /// Set function.
    pub fn set_function(&mut self, name: impl AsRef<str>, function: impl Fn(&Deserializer, &Context, &Placeholder) -> serde_json::Result<Value> + 'static) -> &mut Self {
        self.functions.register(name, function);
        self
    }

    /// Set function.
    pub fn with_function(mut self, name: impl AsRef<str>, function: impl Fn(&Deserializer, &Context, &Placeholder) -> serde_json::Result<Value> + 'static) -> Self {
        self.set_function(name, function);
        self
    }

    /// Override data.
    pub fn with_override(mut self, new_value: Value) -> Self {
        self.override_data(new_value);
        self
    }

    /// Override data.
    pub fn override_data(&mut self, new_value: Value) -> &mut Self {
        self.data.override_recursive(new_value);
        self
    }

    /// Add data.
    pub fn with_additional_data(mut self, new_value: Value) -> Self {
        self.add_data(new_value);
        self
    }

    /// Add data.
    pub fn add_data(&mut self, new_value: Value) -> &mut Self {
        self.data.add_recursive(new_value);
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

    pub(crate) fn set_current_data(&mut self, current: Value) {
        self.current = current;
    }

    /// Find placeholder value.
    pub fn find(&self, deserializer: &Deserializer, placeholder: &Placeholder) -> serde_json::Result<Value> {
        self
            .data
            .get_dot_deserializing(placeholder.path(), deserializer, self)
            .or_else(|_| self.current.get_dot_deserializing(placeholder.path(), deserializer, self))
    }
}