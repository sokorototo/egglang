use super::Operator;
use crate::{
	error::{EggError, EggResult},
	evaluator::evaluate,
	expression::{Expression, Value},
	scope::Scope,
};

// Prints it's data and a newline
pub struct PrintLine;

impl Operator for PrintLine {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		for arg in args {
			match evaluate(arg, scope)? {
				Value::Number(num) => print!("{num}"),
				Value::String(string) => print!("{string}"),
				Value::Nil => print!("Nil"),
				Value::Boolean(b) => print!("{}", if b { "True" } else { "False" }),
				Value::Function(idx) => print!("{:?}", scope.get_function_definition(idx)?),
				Value::Object(tag) => println!("{:?}", scope.get_object(tag)),
			}
		}

		println!();
		Ok(Value::Nil)
	}
}

// Prints it's arguments without a newline
pub struct Print;

impl Operator for Print {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		let len = args.len();
		for (idx, arg) in args.iter().enumerate() {
			match evaluate(arg, scope)? {
				Value::Number(num) => print!("{num}"),
				Value::String(string) => print!("{string}"),
				Value::Nil => print!("Nil"),
				Value::Boolean(b) => print!("{}", if b { "True" } else { "False" }),
				Value::Function(idx) => print!("{:?}", scope.get_function_definition(idx)?),
				Value::Object(tag) => println!("{:?}", scope.get_object(tag)),
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
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		// Print prompt if any
		if let Some(prompt) = args.get(0) {
			Print.evaluate(core::slice::from_ref(prompt), scope)?;
		}

		// read line
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).map_err(|err| EggError::OperatorComplaint(err.to_string()))?;

		Ok(input.trim().into())
	}
}
