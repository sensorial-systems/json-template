//! This module contains the trait implementations for the project.

use std::path::PathBuf;

use serde_json::Value;

use crate::{Context, Deserializer, Path, Placeholder};

/// This trait provides a method to get a value from a JSON object using a dot-separated path.
pub trait GetDot {
    /// Get a value from a JSON object using a dot-separated path, deserializing each segment if needed.
    fn get_dot_deserializing(&self, path: Path, deserializer: &Deserializer, context: &Context) -> serde_json::Result<Value>;
}

impl GetDot for Value {
    fn get_dot_deserializing(&self, path: Path, deserializer: &Deserializer, context: &Context) -> serde_json::Result<Value> {
        path
            .segments()
            .iter()
            .fold(Ok(self.clone()), |acc, segment| {
                if let Some(placeholder) = Placeholder::from_str(segment) {
                    deserializer.resolve_placeholder(&placeholder, context)
                } else {
                    acc?.get(segment)
                        .ok_or_else(|| serde::de::Error::custom(format!("Path not found: {}", path.str())))
                        .and_then(|value| deserializer.resolve_value(value, context))
                }
            })
    }
}

/// Trait to convert to a JSON string.
pub trait ToDeserializable {
    /// Convert to a JSON string.
    fn to_deserializable(&self) -> serde_json::Result<(Option<PathBuf>, Value)>;
}

impl ToDeserializable for std::path::PathBuf {
    fn to_deserializable(&self) -> serde_json::Result<(Option<PathBuf>, Value)> {
        self.as_path().to_deserializable()
    }
}

impl ToDeserializable for std::path::Path {
    fn to_deserializable(&self) -> serde_json::Result<(Option<PathBuf>, Value)> {
        let value = std::fs::read_to_string(self)
            .map_err(|e| serde::de::Error::custom(format!("{} - {}", e, self.display())))?;
        let value = serde_json::from_str(&value)?;
        Ok((self.parent().map(|path| path.to_path_buf()), value))
    }
}

impl ToDeserializable for String {
    fn to_deserializable(&self) -> serde_json::Result<(Option<PathBuf>, Value)> {
        let value = serde_json::from_str(self)?;
        Ok((None, value))
    }
}

impl ToDeserializable for &str {
    fn to_deserializable(&self) -> serde_json::Result<(Option<PathBuf>, Value)> {
        let value = serde_json::from_str(self)?;
        Ok((None, value))
    }
}

impl ToDeserializable for Value {
    fn to_deserializable(&self) -> serde_json::Result<(Option<PathBuf>, Value)> {
        Ok((None, self.clone()))
    }
}