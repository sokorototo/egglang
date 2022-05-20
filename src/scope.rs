use std::{collections::HashMap, sync::Mutex};

use crate::expression::Value;

pub fn build_default_scope() -> Mutex<HashMap<String, Value>> {
    let mut map = HashMap::new();

    map.insert("true".to_string(), true.into());
    map.insert("false".to_string(), false.into());

    map.insert("NUMBER".to_string(), Value::String("__number".to_string()));
    map.insert("STRING".to_string(), Value::String("__string".to_string()));

    Mutex::new(map)
}
