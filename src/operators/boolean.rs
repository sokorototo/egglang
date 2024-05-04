#![allow(clippy::upper_case_acronyms)]

use super::Operator;
use crate::{
	errors::{EggError, EggResult},
	evaluator::evaluate,
	expression::{Expression, Value},
};
use alloc::{
	boxed::Box,
	collections::BTreeMap,
	string::{String, ToString},
};

static NOT_BOOLEAN: &str = "Please provide booleans as arguments for boolean operations";

// AND
pub struct AND;

impl Operator for AND {
	fn evaluate(&self, args: &[Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope, operators)?;
		let val2 = evaluate(&args[1], scope, operators)?;

		match (val1, val2) {
			(Value::Boolean(a), Value::Boolean(b)) => Ok((a && b).into()),
			_ => Err(EggError::OperatorComplaint(NOT_BOOLEAN.to_string())),
		}
	}
}

// AND
pub struct OR;

impl Operator for OR {
	fn evaluate(&self, args: &[Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope, operators)?;
		let val2 = evaluate(&args[1], scope, operators)?;

		match (val1, val2) {
			(Value::Boolean(a), Value::Boolean(b)) => Ok((a || b).into()),
			_ => Err(EggError::OperatorComplaint(NOT_BOOLEAN.to_string())),
		}
	}
}

// AND
pub struct NOT;

impl Operator for NOT {
	fn evaluate(&self, args: &[Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 1);
		match evaluate(&args[0], scope, operators)? {
			Value::Boolean(a) => Ok((!a).into()),
			_ => Err(EggError::OperatorComplaint(NOT_BOOLEAN.to_string())),
		}
	}
}
