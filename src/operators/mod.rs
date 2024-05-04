use crate::{
	errors::EggResult,
	expression::{Expression, Value},
};
use alloc::{boxed::Box, collections::BTreeMap, string::String};

// egg-std definitions
mod arithmetic;
mod boolean;
mod comparison;
mod control_flow;
mod convert;
mod map;

#[cfg(feature = "std")]
mod print;

mod stringtools;
mod variables;

/// Any function callable in Egg
pub trait Operator {
	fn evaluate(&self, args: &[Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value>;
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

	map
}

/// All operations available in Egg
pub fn full<'a>(map: &'a mut BTreeMap<&'static str, Box<dyn Operator>>) -> &'a mut BTreeMap<&'static str, Box<dyn Operator>> {
	minimal(map);

	// Map creation functions
	map.insert("map.new", Box::new(map::NewMap));
	map.insert("map.exists", Box::new(map::ExistsMap));
	map.insert("map.delete", Box::new(map::DeleteMap));

	// Map manipulation functions
	map.insert("map.get", Box::new(map::Get));
	map.insert("map.insert", Box::new(map::Insert));
	map.insert("map.has", Box::new(map::Has));
	map.insert("map.remove", Box::new(map::Remove));
	map.insert("map.size", Box::new(map::Size));

	#[cfg(feature = "std")]
	{
		map.insert("sleep", Box::new(control_flow::Sleep));
		map.insert("map.print", Box::new(map::PrintMap));
		map.insert("print", Box::new(print::Print));
		map.insert("println", Box::new(print::PrintLine));
	}

	map
}
