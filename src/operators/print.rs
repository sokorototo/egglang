use alloc::{boxed::Box, collections::BTreeMap, string::String};

use super::Operator;
use crate::{
	errors::EggResult,
	evaluator::evaluate,
	expression::{Expression, Value},
};

// Prints it's data and a newline
pub struct PrintLine;

impl Operator for PrintLine {
	fn evaluate(&self, args: &[Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		for arg in args {
			match evaluate(arg, scope, builtins)? {
				Value::Number(num) => println!("{num}"),
				Value::String(string) => println!("{string}"),
				Value::Nil => println!("Nil"),
				Value::Boolean(b) => println!("{}", if b { "True" } else { "False" }),
			}
		}

		Ok(Value::Nil)
	}
}

// Prints it's arguments without a newline
pub struct Print;

impl Operator for Print {
	fn evaluate(&self, args: &[Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		for arg in args {
			match evaluate(arg, scope, builtins)? {
				Value::Number(num) => print!("{num} "),
				Value::String(string) => print!("{string} "),
				Value::Nil => print!("Nil"),
				Value::Boolean(b) => print!("{}", if b { "True" } else { "False" }),
			}
		}

		Ok(Value::Nil)
	}
}
