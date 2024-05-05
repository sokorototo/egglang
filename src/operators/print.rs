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
					let function = scope.get_function(idx)?;
					print!("{:?}", function);
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
					let function = scope.get_function(idx)?;
					print!("{:?}", function);
				}
			}

			if len > 1 && idx != len - 1 {
				print!(" ");
			}
		}

		Ok(Value::Nil)
	}
}
