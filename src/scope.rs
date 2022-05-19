use std::{collections::HashMap, sync::Mutex};

use crate::expression::Value;

pub fn build_default_scope<'a>() -> Mutex<HashMap<&'a str, Value>> {
    let mut map = HashMap::new();

    map.insert("true", Value::Isize(1));
    map.insert("false", Value::Isize(0));

	 map.insert("NUMBER", Value::String("__number".to_string()));
	 map.insert("STRING", Value::String("__string".to_string()));

    Mutex::new(map)
}
