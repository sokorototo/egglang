use std::{collections::HashMap, sync::Mutex};

use crate::expression::Value;

pub fn build_default_scope<'a>() -> Mutex<HashMap<&'a str, Value>> {
    let mut map = HashMap::new();

    map.insert("true", Value::Isize(1));
    map.insert("false", Value::Isize(0));

    Mutex::new(map)
}
