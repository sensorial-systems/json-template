//! Functions module.

use std::{collections::HashMap, rc::Rc};

use serde_json::Value;

use crate::{Context, Deserializer, Placeholder};


/// Functions registry.
#[derive(Clone)]
pub struct Functions {
    registry: HashMap<String, Rc<dyn Fn(&Deserializer, &Context, &Placeholder) -> serde_json::Result<Value>>>
}

impl Default for Functions {
    fn default() -> Self {
        let registry = Default::default();
        let mut functions = Functions { registry };
        functions.register("string", |deserializer, context, placeholder| {
            context.find(deserializer, placeholder).map(|value| Value::String(value.to_string()))
        });
        functions.register("file", |deserializer, context, placeholder| {
            context
                .directory
                .as_ref()
                .map(|directory| directory.join(placeholder.path()))
                .ok_or_else(|| serde::de::Error::custom("No directory set."))
                .and_then(|path| deserializer.deserialize::<Value>(path))
        });
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