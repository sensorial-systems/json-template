//! This module contains the trait implementations for the project.

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
