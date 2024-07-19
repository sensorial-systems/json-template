//! Deserializer module.

use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::{Context, Placeholder, ToDeserializable, JSON};

/// A template.
#[derive(Default, Clone, Copy)]
pub struct Deserializer;

impl Deserializer {
    /// Create a new deserializer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Deserialize .
    pub fn deserialize<T: DeserializeOwned>(&self, value: impl ToDeserializable) -> serde_json::Result<T> {
        let context = Context::new();
        self.deserialize_with_context(value, &context)
    }

    /// Deserialize with context.
    pub fn deserialize_with_context<T: DeserializeOwned>(&self, value: impl ToDeserializable, context: &Context) -> serde_json::Result<T> {
        let mut context = context.clone();
        let (directory, value) = value.to_deserializable()?;
        if let (Some(directory), None) = (directory, &context.directory) {
            context.set_directory(Some(directory));
        }
        context.set_current_data(value.clone());
        let value = self.resolve_value(&value, &context)?;
        Ok(serde_json::from_value(serde_json::Value::from(value))?)
    }

    /// Resolve an object.
    pub fn resolve_object(&self, object: &Map<String, Value>, context: &Context) -> serde_json::Result<Value> {
        let mut resolving_object = object.clone();
        for value in resolving_object.values_mut() {
            *value = self.resolve_value(value, context)?;
        }
        Ok(Value::Object(resolving_object))
    }

    /// Resolve a value.
    pub fn resolve_value(&self, value: &Value, context: &Context) -> serde_json::Result<Value> {
        let mut value = value.clone();
        match &value {
            Value::Object(object) => value = self.resolve_object(object, context)?,
            Value::Array(array) => value = self.resolve_array(array, context)?,
            Value::String(string) => value = self.resolve_string(string, context)?,
            _ => {}
        };
        Ok(value)
    }

    /// Resolve a string.
    pub fn resolve_string(&self, string: &str, context: &Context) -> serde_json::Result<Value> {
        let placeholders = Placeholder::placeholders(string);
        if placeholders.len() == 1 {
            if placeholders[0].value == string {
                return self.resolve_placeholder(&placeholders[0], context)
            }
        }
        let string = placeholders.iter().fold(string.to_string(), |acc, placeholder| {
            acc.replace(&placeholder.value, &self.resolve_placeholder(placeholder, context).unwrap().to_text())
        });
        Ok(Value::String(string))
    }

    /// Resolve array.
    pub fn resolve_array(&self, array: &Vec<Value>, context: &Context) -> serde_json::Result<Value> {
        let mut resolving_array = array.clone();
        for value in resolving_array.iter_mut() {
            *value = self.resolve_value(value, context)?;
        }
        Ok(Value::Array(resolving_array))
    }
    
    /// Resolve the placeholder.
    pub fn resolve_placeholder(&self, placeholder: &Placeholder, context: &Context) -> serde_json::Result<Value> {
        let value = if let Some(type_) = placeholder.type_.as_ref() {
            context
                .functions
                .get(type_)
                .ok_or_else(|| serde::de::Error::custom(format!("Function not found: {:?}", placeholder)))
                .and_then(|function| function(self, context, placeholder))
        } else {
            context
                .find(self, &placeholder)
        }?;
        // Resolve placeholders recursively
        self.resolve_value(&value, context)
    }
}
