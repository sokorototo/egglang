#![allow(clippy::upper_case_acronyms)]

use super::Operator;
use crate::{
	errors::{EggError, EggResult},
	evaluator::evaluate,
	expression::{Expression, Value},
};
use std::collections::HashMap;

static NOT_NUMBER_OR_BOOLEAN: &str = "please provide numbers or booleans as arguments for boolean operations";

// AND
pub struct AND;

impl Operator for AND {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope, builtins)?;
		let val2 = evaluate(&args[1], scope, builtins)?;

		match (val1, val2) {
			(Value::Number(a), Value::Number(b)) => Ok((a & b).into()),
			(Value::Boolean(a), Value::Boolean(b)) => Ok((a && b).into()),
			_ => Err(EggError::OperatorComplaint(NOT_NUMBER_OR_BOOLEAN.to_string())),
		}
	}
}

// AND
pub struct OR;

impl Operator for OR {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope, builtins)?;
		let val2 = evaluate(&args[1], scope, builtins)?;

		match (val1, val2) {
			(Value::Number(a), Value::Number(b)) => Ok((a | b).into()),
			(Value::Boolean(a), Value::Boolean(b)) => Ok((a || b).into()),
			_ => Err(EggError::OperatorComplaint(NOT_NUMBER_OR_BOOLEAN.to_string())),
		}
	}
}

// AND
pub struct NOT;

impl Operator for NOT {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 1);
		match evaluate(&args[0], scope, builtins)? {
			Value::Number(a) => Ok((!a).into()),
			Value::Boolean(a) => Ok((!a).into()),
			_ => Err(EggError::OperatorComplaint(NOT_NUMBER_OR_BOOLEAN.to_string())),
		}
	}
}
