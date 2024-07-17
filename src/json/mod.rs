//! JSON utilities.

use regex::Regex;
use serde_json::Value;

use crate::{Template, Placeholder};

/// A JSON value.
#[derive(Default, Clone)]
pub struct Json {
    /// The JSON value.
    pub value: serde_json::Value
}

impl From<serde_json::Value> for Json {
    fn from(value: serde_json::Value) -> Self {
        Self { value }
    }
}

impl From<Json> for serde_json::Value {
    fn from(json: Json) -> Self {
        json.value
    }
}

impl Json {
    /// Create a new JSON value.
    pub fn to_string(&self) -> String {
        if let serde_json::Value::String(value) = &self.value {
            value.clone()
        } else {
            self.value.to_string()
        }
    }

    /// Resolve file placeholders in the JSON value.
    pub fn resolve_files(&mut self, context: &Template) -> serde_json::Result<()> {
        let json = &mut self.value;
        if let Some(object) = json.as_object_mut() {
            for value in object.values_mut() {
                if let Some(string) = value.as_str() {
                    if let Some(placeholder) = Placeholder::from(string) {
                        if placeholder.is_file {
                            if let Some(new_value) = context.resolve(&placeholder) {
                                *value = new_value;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn add_recursive(value: &mut Value, new_value: Value) {
        match value {
            Value::Object(map) => {
                if let Value::Object(new_map) = new_value {
                    for (key, new_value) in new_map {
                        if let Some(value) = map.get_mut(&key) {
                            Self::add_recursive(value, new_value);
                        } else {
                            map.insert(key, new_value);
                        }
                    }
                }
            }
            Value::Array(array) => {
                if let Value::Array(new_array) = new_value {
                    for new_value in new_array.into_iter() {
                        array.push(new_value);
                    }
                }
            }
            _ => *value = new_value
        }
    
    }

    pub(crate) fn override_value_recursive(value: &mut Value, new_value: Value) {
        match value {
            Value::Object(map) => {
                if let Value::Object(new_map) = new_value {
                    for (key, new_value) in new_map {
                        if let Some(value) = map.get_mut(&key) {
                            Self::override_value_recursive(value, new_value);
                        } else {
                            map.insert(key, new_value);
                        }
                    }
                }
            }
            Value::Array(array) => {
                if let Value::Array(new_array) = new_value {
                    for (index, new_value) in new_array.into_iter().enumerate() {
                        if let Some(value) = array.get_mut(index) {
                            Self::override_value_recursive(value, new_value);
                        } else {
                            array.push(new_value);
                        }
                    }
                }
            }
            _ => *value = new_value
        }
    
    }    

    /// Resolve placeholders in the JSON value.
    pub fn resolve_placeholders(&mut self, context: &Template) -> serde_json::Result<()> {
        let json = &mut self.value;
        if let Some(object) = json.as_object_mut() {
            for value in object.values_mut() {
                if let Some(string) = value.as_str() {
                    if let Some(placeholder) = Placeholder::from(string) {
                        if let Some(new_value) = context.resolve(&placeholder) {
                            *value = new_value;
                        }
                    } else { // It's a format string.
                        let re = Regex::new(r"\{([^}]+)\}").unwrap();
                        let matches = re
                            .captures_iter(string)
                            .filter_map(|cap| cap.get(0))
                            .map(|cap| cap.as_str())
                            .filter_map(|placeholder| Placeholder::from(placeholder))
                            .filter_map(|placeholder| context
                                .resolve(&placeholder)
                                .map(|value| (placeholder, value))
                            ).collect::<Vec<_>>();
                        *value = Value::String(
                            matches
                                .iter()
                                .fold(string.to_string(), |acc, (placeholder, value)|
                                    acc.replace(&placeholder.value, &Json::from(value.clone()).to_string())
                                )
                        );
                    }
                }
            }
        }
        Ok(())
    }
}
