//! Placeholder module.

use crate::Path;

/// This struct represents a placeholder in a JSON object.
#[derive(Debug, Clone)]
pub struct Placeholder {
    /// The placeholder value.
    pub value: String,
    /// The placeholder type.
    pub type_: Option<String>,
}

impl Placeholder {
    /// Get all the placeholders in a string.
    /// If value == "{time} {time:3}", then placeholders == ["{time}", "{time:3}"].
    /// If value == "{time:{time:5}}  {time}", then placeholders == ["{time:{time:5}}", "{time}"].
    pub fn placeholders(value: &str) -> Vec<Self> {
        let mut levels = 0;
        let mut current_placeholder = String::new();
        let mut placeholders = Vec::new();
        for character in value.chars() {
            match character {
                '{' => {
                    if levels == 0 {
                        current_placeholder.clear();
                    }
                    levels += 1;
                    current_placeholder.push(character);
                }
                '}' => {
                    levels -= 1;
                    current_placeholder.push(character);
                    if levels == 0 {
                        placeholders.push(Self::from_str(&current_placeholder).expect("Failed to create placeholder."));
                    }
                }
                _ => {
                    if levels > 0 {
                        current_placeholder.push(character);
                    }
                }
            }
        }
        placeholders
    }

    /// Create a new placeholder from a string.
    pub fn from_str(value: &str) -> Option<Self> {
        if value.starts_with('{') && value.ends_with('}') {
            let value = value.to_string();
            let chars = value.chars();
            let first = chars.clone().nth(0);
            let second = chars.clone().nth(1);
            let type_ = first.zip(second).map(|(first, second)| {
                if first == '{' && second.is_alphanumeric() {
                    value.find(':').map(|index| value[1 .. index].to_string())
                } else {
                    None
                }
            }).flatten();
            Some(Self { value, type_ })
        } else {
            None
        }
    }

    /// Get the path of the placeholder.
    pub fn path(&self) -> Path {
        if let Some(type_) = &self.type_ {
            Path::new(&self.value[type_.len() + 2 .. self.value.len() - 1])
        } else {
            Path::new(&self.value[1 .. self.value.len() - 1])
        }
    }
}

