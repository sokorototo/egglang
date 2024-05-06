use crate::{
	error::EggResult,
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

/// Trait for functions defined Rust, callable in Egg.
/// Operators then need to be registered into the operators map, during script [evaluation](crate::evaluator::evaluate).
pub trait Operator {
	/// Invokes this Operator.
	///
	/// `[args]` are the arguments to the Operator, as [`Expressions`](Expression). To get a [`Value`] from an argument, use the [`evaluate`](crate::evaluator::evaluate) function.
	///
	/// `[scope]` is the current scope, where variables are stored. A local scope if the function is called from a user-defined function, or the global scope if called from the main script.
	///
	/// `[operators]` is a map of all other operators; Can be used directly, but it's main use is to invoke [`evaluate`](crate::evaluator::evaluate) on arguments.
	///
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value>;
}

/// Create an empty map of operations
pub fn empty() -> BTreeMap<&'static str, Box<dyn Operator>> {
	BTreeMap::new()
}

/// Only the basic operations available in Egg
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

/// Create, interact with and delete objects
pub fn object_tools(map: &mut BTreeMap<&'static str, Box<dyn Operator>>) {
	map.insert("object.new", Box::new(scope::object::CreateObject));
	map.insert("object.get", Box::new(scope::object::Get));
	map.insert("object.insert", Box::new(scope::object::Insert));
	map.insert("object.has", Box::new(scope::object::Has));
	map.insert("object.remove", Box::new(scope::object::Remove));
	map.insert("object.size", Box::new(scope::object::Size));
	map.insert("object.clear", Box::new(scope::object::Clear));
}

/// Strings tools
pub fn string_tools<'a>(map: &'a mut BTreeMap<&'static str, Box<dyn Operator>>) {
	map.insert("string.length", Box::new(stringtools::Length));
	map.insert("string.slice", Box::new(stringtools::Slice));
	map.insert("string.concat", Box::new(stringtools::Concat));
	map.insert("string.to_upper", Box::new(stringtools::ToUpper));
	map.insert("string.to_lower", Box::new(stringtools::ToLower));
	map.insert("string.trim", Box::new(stringtools::Trim));
}

/// Console Functions
#[cfg(feature = "std")]
pub fn console_tools<'a>(map: &'a mut BTreeMap<&'static str, Box<dyn Operator>>) {
	map.insert("print", Box::new(console::Print));
	map.insert("println", Box::new(console::PrintLine));
	map.insert("readline", Box::new(console::ReadLine));
	map.insert("object.print", Box::new(scope::object::PrintObject));
}

/// All Internal functions defined in `Egg`
pub fn full<'a>(map: &'a mut BTreeMap<&'static str, Box<dyn Operator>>) {
	minimal(map);
	object_tools(map);
	string_tools(map);

	#[cfg(feature = "std")]
	{
		map.insert("sleep", Box::new(control_flow::Sleep));
		console_tools(map);
	}
}
