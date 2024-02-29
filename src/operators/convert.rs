use std::collections::HashMap;

use super::Operator;
use crate::{
	errors::{EggError, EggResult},
	evaluator::evaluate,
	expression::{self, Value},
};

// Operation that converts numbers to strings
pub struct ToString;

impl Operator for ToString {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Evaluate
		let res = evaluate(&args[0], scope, builtins)?;
		Ok(Value::String(res.to_string().into()))
	}
}

// Operation that converts strings to numbers
pub struct ToNumber;

impl Operator for ToNumber {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Evaluate
		let res = evaluate(&args[0], scope, builtins)?;

		match res {
			Value::String(string) => string.parse::<isize>().map(Value::Number).map_err(EggError::UnableToParseNumber),
			Value::Number(n) => Ok(Value::Number(n)),
			Value::Nil => Err(EggError::OperatorComplaint("Can't convert Nil to a number".to_string())),
			Value::Boolean(b) => Ok(Value::Number(b as isize)),
		}
	}
}
