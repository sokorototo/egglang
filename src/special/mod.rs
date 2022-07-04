use crate::expression::{Expression, Value};
use std::collections::HashMap;

// Various special, forms
mod arithmetic;
mod boolean;
mod comparison;
mod control_flow;
mod convert;
mod memory;
mod print;
mod stringtools;

pub trait SpecialForm<'a> {
    fn evaluate(
        &self,
        args: &'a [Expression],
        scope: &mut HashMap<String, Value>,
        special_forms: &HashMap<&'a str, Box<dyn SpecialForm<'a> + 'a>>,
    ) -> Value;
}

// My Egg implementation's standard library
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
    map.insert("print", Box::new(print::Print));

    // Control flow
    map.insert("if", Box::new(control_flow::If));
    map.insert("do", Box::new(control_flow::Do));
    map.insert("while", Box::new(control_flow::While));
    map.insert("repeat", Box::new(control_flow::Repeat));
    map.insert("sleep", Box::new(control_flow::Sleep));
    map.insert("panic", Box::new(control_flow::Panic));
    map.insert("assert", Box::new(control_flow::Assert));

    // Comparison
    map.insert("equals", Box::new(comparison::Equals));
    map.insert("not_equals", Box::new(comparison::NotEquals));
    map.insert("greater_than", Box::new(comparison::GreaterThan));
    map.insert("less_than", Box::new(comparison::LessThan));
    map.insert("is_nil", Box::new(comparison::IsNil));

    // Arithmetic
    map.insert("add", Box::new(arithmetic::Add));
    map.insert("subtract", Box::new(arithmetic::Subtract));
    map.insert("divide", Box::new(arithmetic::Divide));
    map.insert("multiply", Box::new(arithmetic::Multiply));

    // Boolean
    map.insert("and", Box::new(boolean::AND));
    map.insert("or", Box::new(boolean::OR));
    map.insert("not", Box::new(boolean::NOT));

    // String tools
    map.insert("string_length", Box::new(stringtools::Length));
    map.insert("string_slice", Box::new(stringtools::Slice));
    map.insert("string_concat", Box::new(stringtools::Concat));
    map.insert("string_to_upper", Box::new(stringtools::ToUpper));
    map.insert("string_to_lower", Box::new(stringtools::ToLower));
    map.insert("string_trim", Box::new(stringtools::Trim));

    // Conversion forms
    map.insert("str", Box::new(convert::ToString));
    map.insert("num", Box::new(convert::ToNumber));
    map
}

