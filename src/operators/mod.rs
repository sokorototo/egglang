use crate::{
	errors::EggResult,
	expression::{Expression, Value},
	scope::{self, Scope},
};
use alloc::{boxed::Box, collections::BTreeMap};

// egg-std definitions
mod arithmetic;
mod boolean;
mod comparison;
mod control_flow;
mod convert;

#[cfg(feature = "std")]
mod console;

mod stringtools;
mod variables;

/// Any function callable in Egg
pub trait Operator {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value>;
}

/// Create an empty map of operations
pub fn empty() -> BTreeMap<&'static str, Box<dyn Operator>> {
	BTreeMap::new()
}

/// Only the basic operations available in Egg. No print, stringtools or maps
pub fn minimal<'a>(map: &'a mut BTreeMap<&'static str, Box<dyn Operator>>) -> &'a mut BTreeMap<&'static str, Box<dyn Operator>> {
	// Insert language statements
	map.insert("define", Box::new(variables::Define));
	map.insert("set", Box::new(variables::Set));
	map.insert("delete", Box::new(variables::Delete));
	map.insert("exists", Box::new(variables::Exists));
	map.insert("typeof", Box::new(variables::TypeOf));

	// Control flow
	map.insert("if", Box::new(control_flow::If));
	map.insert("do", Box::new(control_flow::Do));
	map.insert("while", Box::new(control_flow::While));
	map.insert("repeat", Box::new(control_flow::Repeat));
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

	// Boolean
	map.insert("and", Box::new(boolean::AND));
	map.insert("or", Box::new(boolean::OR));
	map.insert("not", Box::new(boolean::NOT));

	// Type conversion functions
	map.insert("str", Box::new(convert::ToString));
	map.insert("num", Box::new(convert::ToNumber));

	// Function creation
	map.insert("fn", Box::new(scope::functions::CreateFunction));

	map
}

/// All operations available in Egg
pub fn full<'a>(map: &'a mut BTreeMap<&'static str, Box<dyn Operator>>) -> &'a mut BTreeMap<&'static str, Box<dyn Operator>> {
	minimal(map);

	// Map creation functions
	map.insert("map.new", Box::new(scope::map::NewMap));
	map.insert("map.exists", Box::new(scope::map::ExistsMap));
	map.insert("map.delete", Box::new(scope::map::DeleteMap));

	// Map manipulation functions
	map.insert("map.get", Box::new(scope::map::Get));
	map.insert("map.insert", Box::new(scope::map::Insert));
	map.insert("map.has", Box::new(scope::map::Has));
	map.insert("map.remove", Box::new(scope::map::Remove));
	map.insert("map.size", Box::new(scope::map::Size));
	map.insert("map.clear", Box::new(scope::map::Clear));

	// String tools
	map.insert("string.length", Box::new(stringtools::Length));
	map.insert("string.slice", Box::new(stringtools::Slice));
	map.insert("string.concat", Box::new(stringtools::Concat));
	map.insert("string.to_upper", Box::new(stringtools::ToUpper));
	map.insert("string.to_lower", Box::new(stringtools::ToLower));
	map.insert("string.trim", Box::new(stringtools::Trim));

	// Type conversion functions
	map.insert("str", Box::new(convert::ToString));
	map.insert("num", Box::new(convert::ToNumber));

	#[cfg(feature = "std")]
	{
		map.insert("sleep", Box::new(control_flow::Sleep));
		map.insert("map.print", Box::new(scope::map::PrintMap));
		map.insert("print", Box::new(console::Print));
		map.insert("println", Box::new(console::PrintLine));
		map.insert("readline", Box::new(console::ReadLine));
	}

	map
}
