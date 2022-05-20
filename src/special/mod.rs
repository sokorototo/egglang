use crate::expression::{Expression, Value};
use std::{collections::HashMap, rc::Rc, sync::Mutex};

// Various special, forms
mod arithmetic;
mod boolean;
mod comparison;
mod control_flow;
mod memory;
mod print;

pub trait SpecialForm<'a> {
    fn evaluate(
        &self,
        args: &'a [Expression],
        scope: &Mutex<HashMap<Rc<str>, Value>>,
        special_forms: &mut HashMap<&'a str, Box<dyn SpecialForm<'a> + 'a>>,
    ) -> Value;
}

pub fn build_special_forms<'a>() -> HashMap<&'static str, Box<dyn SpecialForm<'a>>> {
    let mut map: HashMap<&'static str, Box<dyn SpecialForm<'a>>> = HashMap::new();

    // Insert language statements
    map.insert("define", Box::new(memory::Define));
    map.insert("mutate", Box::new(memory::Mutate));
    map.insert("delete", Box::new(memory::Delete));
    map.insert("exists", Box::new(memory::Exists));
    map.insert("typeof", Box::new(memory::TypeOf));

    // Printing
    map.insert("print_line", Box::new(print::PrintLine));
    map.insert("print", Box::new(print::PrintLine));

    // Control flow
    map.insert("if", Box::new(control_flow::If));
    map.insert("do", Box::new(control_flow::Do));
    map.insert("while", Box::new(control_flow::While));
    map.insert("repeat", Box::new(control_flow::Repeat));

    // Comparison
    map.insert("equals", Box::new(comparison::Equals));
    map.insert("not_equals", Box::new(comparison::NotEquals));
    map.insert("greater_than", Box::new(comparison::GreaterThan));
    map.insert("less_than", Box::new(comparison::LessThan));

    // Arithmetic
    map.insert("add", Box::new(arithmetic::Add));
    map.insert("subtract", Box::new(arithmetic::Subtract));
    map.insert("divide", Box::new(arithmetic::Divide));
    map.insert("multiply", Box::new(arithmetic::Multiply));

    // Boolean
    map.insert("and", Box::new(boolean::AND));
    map.insert("or", Box::new(boolean::OR));
    map.insert("not", Box::new(boolean::NOT));

    map
}
