use std::collections::HashMap;

use crate::expression::Value;

/// Build a new scope containing values required by the runtime.
pub fn default() -> HashMap<String, Value> {
    let mut map = HashMap::new();

    map.insert("true".into(), true.into());
    map.insert("false".into(), false.into());

    // Globals identifying type
    map.insert("NUMBER".into(), Value::String("__NUMBER".into()));
    map.insert("STRING".into(), Value::String("__STRING".into()));

    map
}

// Create an empty scope
pub fn empty() -> HashMap<String, Value> {
    HashMap::new()
}
