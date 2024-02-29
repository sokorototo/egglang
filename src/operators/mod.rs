use crate::{
	errors::EggResult,
	expression::{Expression, Value},
};
use std::collections::HashMap;

// egg-std definitions
mod arithmetic;
mod boolean;
mod comparison;
mod control_flow;
mod convert;
mod map;
mod memory;
mod print;
mod stringtools;

/// Any function callable in Egg
pub trait Operator {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value>;
}

// My Egg implementation's standard library
pub fn std() -> HashMap<&'static str, Box<dyn Operator>> {
	let mut map: HashMap<&'static str, Box<dyn Operator>> = HashMap::new();

	// Insert language statements
	map.insert("define", Box::new(memory::Define));
	map.insert("set", Box::new(memory::Set));
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
	map.insert("sum", Box::new(arithmetic::Sum));
	map.insert("subtract", Box::new(arithmetic::Subtract));
	map.insert("divide", Box::new(arithmetic::Divide));
	map.insert("multiply", Box::new(arithmetic::Multiply));
	map.insert("modulus", Box::new(arithmetic::Modulus));

	// Arithmetic symbols
	map.insert("+", Box::new(arithmetic::Sum));
	map.insert("-", Box::new(arithmetic::Subtract));
	map.insert("/", Box::new(arithmetic::Divide));
	map.insert("*", Box::new(arithmetic::Multiply));
	map.insert("%", Box::new(arithmetic::Modulus));

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

	// Type conversion functions
	map.insert("str", Box::new(convert::ToString));
	map.insert("num", Box::new(convert::ToNumber));

	// Map (ie dictionary) functions
	map.insert("map.new", Box::new(map::NewMap));
	map.insert("map.exists", Box::new(map::ExistsMap));
	map.insert("map.delete", Box::new(map::DeleteMap));
	map.insert("map.print", Box::new(map::PrintMap));

	map.insert("map.get", Box::new(map::Get));
	map.insert("map.insert", Box::new(map::Insert));
	map.insert("map.has", Box::new(map::Has));
	map.insert("map.remove", Box::new(map::Remove));
	map.insert("map.size", Box::new(map::Size));

	map
}
