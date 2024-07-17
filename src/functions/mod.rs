//! Functions module.

use std::{collections::HashMap, rc::Rc};

use serde_json::Value;

use crate::{GetDot, Placeholder, Template};


/// Functions registry.
#[derive(Clone)]
pub struct Functions {
    registry: HashMap<String, Rc<dyn Fn(&Template, &Placeholder) -> Option<Value>>>
}

impl Default for Functions {
    fn default() -> Self {
        let registry = Default::default();
        let mut functions = Functions { registry };
        functions.register("string", |template, placeholder| {
            template.data.value.get_dot(placeholder.path()).cloned().map(|value| Value::String(value.to_string()))
        });
        functions.register("file", |template, placeholder| {
            template
                .directory
                .as_ref()
                .map(|directory| directory.join(placeholder.path()))
                .and_then(|path| Template::new().deserialize::<Value>(path).ok())
        });
        functions
    }    
}

impl Functions {
    /// Create a new functions.
    pub fn register(&mut self, name: impl AsRef<str>, function: impl Fn(&Template, &Placeholder) -> Option<Value> + 'static) {
        self.registry.insert(name.as_ref().to_string(), Rc::new(function));
    }

    /// Get a function.
    pub fn get(&self, name: impl AsRef<str>) -> Option<Rc<dyn Fn(&Template, &Placeholder) -> Option<Value>>> {
        self.registry.get(name.as_ref()).cloned()
    }
}