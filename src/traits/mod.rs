//! This module contains the trait implementations for the project.

use std::path::PathBuf;

use serde_json::Value;

/// This trait provides a method to get a value from a JSON object using a dot-separated path.
pub trait GetDot {
    /// Get a value from a JSON object using a dot-separated path.
    fn get_dot(&self, path: &str) -> Option<&Value>;
}

impl GetDot for Value {
    fn get_dot(&self, path: &str) -> Option<&Value> {
        path
            .split(".")
            .fold(Some(self), |acc, segment|
                acc?.get(segment)
            )
    }
}

/// Trait to convert to a JSON string.
pub trait ToDeserializable {
    /// Convert to a JSON string.
    fn to_deserializable(&self) -> serde_json::Result<(Option<PathBuf>, String)>;
}

impl ToDeserializable for std::path::PathBuf {
    fn to_deserializable(&self) -> serde_json::Result<(Option<PathBuf>, String)> {
        self.as_path().to_deserializable()
    }
}

impl ToDeserializable for std::path::Path {
    fn to_deserializable(&self) -> serde_json::Result<(Option<PathBuf>, String)> {
        let value = std::fs::read_to_string(self)
            .map_err(|e| serde::de::Error::custom(format!("{} - {}", e, self.display())))?;
        Ok((self.parent().map(|path| path.to_path_buf()), value))
    }
}

impl ToDeserializable for String {
    fn to_deserializable(&self) -> serde_json::Result<(Option<PathBuf>, String)> {
        Ok((None, self.clone()))
    }
}

impl ToDeserializable for &str {
    fn to_deserializable(&self) -> serde_json::Result<(Option<PathBuf>, String)> {
        Ok((None, self.to_string()))
    }
}