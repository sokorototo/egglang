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

// Checks for equality
pub struct Equals;

impl super::Operator for Equals {
	fn evaluate(&self, args: &[Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope, operators)?;
		let val2 = evaluate(&args[1], scope, operators)?;

		Ok((val1 == val2).into())
	}
}

// Checks for inequality
pub struct NotEquals;

impl super::Operator for NotEquals {
	fn evaluate(&self, args: &[Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope, operators)?;
		let val2 = evaluate(&args[1], scope, operators)?;

		Ok((val1 != val2).into())
	}
}

// Greater than
pub struct GreaterThan;

impl super::Operator for GreaterThan {
	fn evaluate(&self, args: &[Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope, operators)?;
		let val2 = evaluate(&args[1], scope, operators)?;

		match (val1, val2) {
			(Value::Number(a), Value::Number(b)) => Ok((a > b).into()),
			_ => Err(EggError::OperatorComplaint("please provide numbers as arguments for mathematical operations".to_string())),
		}
	}
}

// Greater than
pub struct LessThan;

impl super::Operator for LessThan {
	fn evaluate(&self, args: &[Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope, operators)?;
		let val2 = evaluate(&args[1], scope, operators)?;

		match (val1, val2) {
			(Value::Number(a), Value::Number(b)) => Ok((a < b).into()),
			_ => Err(EggError::OperatorComplaint("please provide numbers as arguments for mathematical operations".to_string())),
		}
	}
}

// Write a form that checks if a value is nil, returns a boolean if true
pub struct IsNil;

impl super::Operator for IsNil {
	fn evaluate(&self, args: &[Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 1);

		Ok(matches!(evaluate(&args[0], scope, operators)?, Value::Nil).into())
	}
}
