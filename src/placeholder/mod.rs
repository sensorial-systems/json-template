//! Placeholder module.

use serde_json::Value;

use crate::JSON;

/// This struct represents a placeholder in a JSON object.
#[derive(Debug, Clone)]
pub struct Placeholder {
    /// The placeholder value.
    pub value: String,
    /// The placeholder type.
    pub type_: Option<String>,
}

impl Placeholder {
    /// Create a new placeholder from a string.
    pub fn from_str(value: &str) -> Option<Self> {
        if value.starts_with('{') && value.ends_with('}') {
            let value = value.to_string();
            let type_ = value.find(':').map(|index| value[1 .. index].to_string());
            Some(Self { value, type_ })
        } else {
            None
        }
    }

    /// Create a new placeholder from a Value.
    pub fn from_value(value: &Value) -> Option<Self> {
        Self::from_str(&value.to_text())
    }

    /// Get the path of the placeholder.
    pub fn path(&self) -> &str {
        if let Some(type_) = &self.type_ {
            &self.value[type_.len() + 2 .. self.value.len() - 1]
        } else {
            &self.value[1 .. self.value.len() - 1]
        }
    }
}

