//! JSON utilities.

use serde_json::Value;

/// A JSON value.
#[derive(Default, Debug, Clone)]
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
}
