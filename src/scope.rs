use std::{collections::HashMap, sync::Mutex};

use crate::expression::Value;

pub fn build_default_scope<'a>() -> Mutex<HashMap<&'a str, Value>> {
    Mutex::new(HashMap::new())
}
