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

    // Printing
    map.insert("print_line", Box::new(print::PrintLine));
    map.insert("print", Box::new(print::PrintLine));

    // Control flow
    map.insert("if", Box::new(control_flow::If));
    map.insert("do", Box::new(control_flow::Do));
    map.insert("while", Box::new(control_flow::While));
    map.insert("repeat", Box::new(control_flow::Repeat));

    // Type checking
    map.insert("typeof", Box::new(operators::TypeOf));

    // Comparison
    map.insert("equals", Box::new(operators::Equals));
    map.insert("not_equals", Box::new(operators::NotEquals));
    map.insert("greater_than", Box::new(operators::GreaterThan));
    map.insert("less_than", Box::new(operators::LessThan));

    // Arithmetic
    map.insert("add", Box::new(operators::Add));
    map.insert("subtract", Box::new(operators::Subtract));
    map.insert("divide", Box::new(operators::Divide));
    map.insert("multiply", Box::new(operators::Multiply));

    map
}
