use crate::expression::{Expression, Value};
use std::{collections::HashMap, sync::Mutex};

// Various special, forms
mod control_flow;
mod define;
mod operators;
mod print;

pub trait SpecialForm<'a> {
    fn evaluate(
        &self,
        args: &'a [Expression],
        scope: &'a Mutex<HashMap<&'a str, Value>>,
        special_forms: &HashMap<&'a str, Box<dyn SpecialForm<'a> + 'a>>,
    ) -> Value;
}

pub fn build_special_forms<'a>() -> HashMap<&'static str, Box<dyn SpecialForm<'a>>> {
    let mut map: HashMap<&'static str, Box<dyn SpecialForm<'a>>> = HashMap::new();

    // Insert language statements
    map.insert("define", Box::new(define::Define));

    map.insert("printline", Box::new(print::PrintLine));
    map.insert("print", Box::new(print::PrintLine));

    map.insert("if", Box::new(control_flow::If));
    map.insert("do", Box::new(control_flow::Do));
    map.insert("while", Box::new(control_flow::While));
    map.insert("repeat", Box::new(control_flow::Repeat));

    map.insert("equals", Box::new(operators::Equals));
    map.insert("typeof", Box::new(operators::TypeOf));

    map
}
