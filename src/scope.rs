use std::{collections::HashMap, rc::Rc};

use crate::expression::Value;

pub fn build_default_scope() -> HashMap<Rc<str>, Value> {
    let mut map = HashMap::new();

    map.insert("true".into(), true.into());
    map.insert("false".into(), false.into());

    // Globals identifying type
    map.insert("NUMBER".into(), Value::String("__number".into()));
    map.insert("STRING".into(), Value::String("__string".into()));

    map
}
