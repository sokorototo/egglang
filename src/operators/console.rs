use std::collections::BTreeMap;

use super::Operator;
use crate::{
	errors::EggResult,
	evaluator::evaluate,
	expression::{Expression, Value},
	scope::Scope,
};

// Prints it's data and a newline
pub struct PrintLine;

impl Operator for PrintLine {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		for arg in args {
			match evaluate(arg, scope, operators)? {
				Value::Number(num) => print!("{num}"),
				Value::String(string) => print!("{string}"),
				Value::Nil => print!("Nil"),
				Value::Boolean(b) => print!("{}", if b { "True" } else { "False" }),
				Value::Function(idx) => {
					let function = scope.get_function_definition(idx)?;
					print!("{:?}", function);
				}
				Value::Object(tag) => {
					let map = scope.get_map(tag)?;
					println!("{:?}", map);
				}
			}
		}

		println!();
		Ok(Value::Nil)
	}
}

// Prints it's arguments without a newline
pub struct Print;

impl Operator for Print {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		let len = args.len();
		for (idx, arg) in args.iter().enumerate() {
			match evaluate(arg, scope, operators)? {
				Value::Number(num) => print!("{num}"),
				Value::String(string) => print!("{string}"),
				Value::Nil => print!("Nil"),
				Value::Boolean(b) => print!("{}", if b { "True" } else { "False" }),
				Value::Function(idx) => {
					let function = scope.get_function_definition(idx)?;
					print!("{:?}", function);
				}
				Value::Object(tag) => {
					let map = scope.get_map(tag)?;
					println!("{:?}", map);
				}
			}

			if len > 1 && idx != len - 1 {
				print!(" ");
			}
		}

		Ok(Value::Nil)
	}
}

/// Reads a line of input from the console
pub struct ReadLine;

impl Operator for ReadLine {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// Print prompt if any
		if let Some(prompt) = args.get(0) {
			Print.evaluate(core::slice::from_ref(prompt), scope, operators)?;
		}

		// read line
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).map_err(|err| crate::errors::EggError::OperatorComplaint(err.to_string()))?;

		Ok(input.trim().into())
	}
}
