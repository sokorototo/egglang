use std::collections::HashMap;

use crate::expression::Value;

pub fn build_default_scope() -> HashMap<String, Value> {
    let mut map = HashMap::new();

    map.insert("true".into(), true.into());
    map.insert("false".into(), false.into());

    // Globals identifying type
    map.insert("NUMBER".into(), Value::String("__NUMBER".into()));
    map.insert("STRING".into(), Value::String("__STRING".into()));

    map
}
