use std::{collections::BTreeMap, io::Write};

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
		let len = args.len();
		let mut lock = std::io::stdout().lock();

		for (idx, arg) in args.iter().enumerate() {
			match evaluate(arg, scope, builtins)? {
				Value::Number(num) => lock.write_all(format!("{num}").as_bytes()).unwrap(),
				Value::String(string) => lock.write_all(string.as_bytes()).unwrap(),
				Value::Nil => lock.write_all(b"Nil").unwrap(),
				Value::Boolean(b) => lock.write_all(if b { b"True" } else { b"False" }).unwrap(),
			};

			if len > 1 && idx != len - 1 {
				lock.write_all(b" ").unwrap();
			}
		}

		lock.write_all(b"\n").unwrap();
		lock.flush().unwrap();
		Ok(Value::Nil)
	}
}

// Prints it's arguments without a newline
pub struct Print;

impl Operator for Print {
	fn evaluate(&self, args: &[Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		let len = args.len();
		let mut lock = std::io::stdout().lock();

		for (idx, arg) in args.iter().enumerate() {
			match evaluate(arg, scope, builtins)? {
				Value::Number(num) => lock.write_all(format!("{num}").as_bytes()).unwrap(),
				Value::String(string) => lock.write_all(string.as_bytes()).unwrap(),
				Value::Nil => lock.write_all(b"Nil").unwrap(),
				Value::Boolean(b) => lock.write_all(if b { b"True" } else { b"False" }).unwrap(),
			};

			if len > 1 && idx != len - 1 {
				lock.write_all(b" ").unwrap();
			}
		}

		lock.flush().unwrap();
		Ok(Value::Nil)
	}
}
