//! Placeholder module.

/// This struct represents a placeholder in a JSON object.
pub struct Placeholder {
    /// The placeholder value.
    pub value: String,
    /// Whether the placeholder is a string.
    pub is_str: bool,
    /// Whether the placeholder is a file.
    pub is_file: bool,
}

impl Placeholder {
    /// Create a new placeholder from a string.
    pub fn from(value: &str) -> Option<Self> {
        if value.starts_with('{') && value.ends_with('}') {
            let value = value.to_string();
            let is_str = value.starts_with("{string:");
            let is_file = value.starts_with("{file:");
            Some(Self { value, is_str, is_file })
        } else {
            None
        }
    }

    /// Get the path of the placeholder.
    pub fn path(&self) -> &str {
        if self.is_str {
            &self.value[8 .. self.value.len() - 1]
        } else if self.is_file {
            &self.value[6 .. self.value.len() - 1]
        } else {
            &self.value[1 .. self.value.len() - 1]
        }
    }
}

