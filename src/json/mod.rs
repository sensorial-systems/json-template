//! JSON utilities.

use serde_json::Value;

/// JSON utilities.
pub trait JSON {
    /// Text without quotes (specially for strings).
    fn to_text(&self) -> String;

    /// Add a value recursively.
    fn add_recursive(&mut self, new_value: Value);

    /// Override a value recursively.
    fn override_recursive(&mut self, new_value: Value);
}

impl JSON for Value {
    fn to_text(&self) -> String {
        if let serde_json::Value::String(value) = self {
            value.clone()
        } else {
            self.to_string()
        }
    }

    fn add_recursive(&mut self, new_value: Value) {
        let value = self;
        match value {
            Value::Object(map) => {
                if let Value::Object(new_map) = new_value {
                    for (key, new_value) in new_map {
                        if let Some(value) = map.get_mut(&key) {
                            value.add_recursive(new_value);
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

    fn override_recursive(&mut self, new_value: Value) {
        let value = self;
        match value {
            Value::Object(map) => {
                if let Value::Object(new_map) = new_value {
                    for (key, new_value) in new_map {
                        if let Some(value) = map.get_mut(&key) {
                            value.override_recursive(new_value);
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
                            value.override_recursive(new_value);
                        } else {
                            array.push(new_value);
                        }
                    }
                }
            }
            _ => *value = new_value
        }
    }
}
