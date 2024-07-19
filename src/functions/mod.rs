//! Functions module.

use std::{collections::HashMap, rc::Rc};

use serde_json::Value;

use crate::{Context, Deserializer, Placeholder, JSON};


/// Functions registry.
#[derive(Clone)]
pub struct Functions {
    registry: HashMap<String, Rc<dyn Fn(&Deserializer, &Context, &Placeholder) -> serde_json::Result<Value>>>
}

/// Transforms everything into a string.
fn string(deserializer: &Deserializer, context: &Context, placeholder: &Placeholder) -> serde_json::Result<Value> {
    context.find(deserializer, placeholder).map(|value| Value::String(value.to_string()))
}

/// Reads a file.
fn file(deserializer: &Deserializer, context: &Context, placeholder: &Placeholder) -> serde_json::Result<Value> {
    context
        .directory()
        .as_ref()
        .map(|directory| directory.join(placeholder.path().str()))
        .ok_or_else(|| serde::de::Error::custom("No directory set."))
        .and_then(|path| deserializer.deserialize_with_context::<Value>(path, context))
}

/// Composes a value from multiple placeholders.
fn compose(deserializer: &Deserializer, context: &Context, placeholder: &Placeholder) -> serde_json::Result<Value> {
    let path = placeholder.path();
    let parts = path.str().split(',').collect::<Vec<_>>();
    let mut value = Value::Object(Default::default());
    for part in parts {
        let new_value = deserializer.resolve_string(part, context)?;
        value.add_recursive(new_value);
    }
    Ok(value)
}

impl Default for Functions {
    fn default() -> Self {
        let registry = Default::default();
        let mut functions = Functions { registry };
        functions.register("string", string);
        functions.register("file", file);
        functions.register("compose", compose);
        functions
    }    
}

impl Functions {
    /// Create a new functions.
    pub fn register(&mut self, name: impl AsRef<str>, function: impl Fn(&Deserializer, &Context, &Placeholder) -> serde_json::Result<Value> + 'static) {
        self.registry.insert(name.as_ref().to_string(), Rc::new(function));
    }

    /// Get a function.
    pub fn get(&self, name: impl AsRef<str>) -> Option<Rc<dyn Fn(&Deserializer, &Context, &Placeholder) -> serde_json::Result<Value>>> {
        self.registry.get(name.as_ref()).cloned()
    }
}