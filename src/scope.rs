use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::expression::Value;

pub fn build_default_scope() -> RefCell<HashMap<Rc<str>, Value>> {
    let mut map = HashMap::new();

    map.insert("true".into(), true.into());
    map.insert("false".into(), false.into());

    map.insert("NUMBER".into(), Value::String("__number".into()));
    map.insert("STRING".into(), Value::String("__string".into()));

    RefCell::new(map)
}
