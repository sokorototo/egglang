#![allow(clippy::upper_case_acronyms)]

use super::Operator;
use crate::{
	error::{EggError, EggResult},
	evaluator::evaluate,
	expression::{Expression, Value},
	scope::Scope,
};
use alloc::string::ToString;

static NOT_BOOLEAN: &str = "Please provide booleans as arguments for boolean operations";

// AND
pub struct AND;

impl Operator for AND {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope)?;
		let val2 = evaluate(&args[1], scope)?;

		match (val1, val2) {
			(Value::Boolean(a), Value::Boolean(b)) => Ok((a && b).into()),
			_ => Err(EggError::OperatorComplaint(NOT_BOOLEAN.to_string())),
		}
	}
}

// AND
pub struct OR;

impl Operator for OR {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope)?;
		let val2 = evaluate(&args[1], scope)?;

		match (val1, val2) {
			(Value::Boolean(a), Value::Boolean(b)) => Ok((a || b).into()),
			_ => Err(EggError::OperatorComplaint(NOT_BOOLEAN.to_string())),
		}
	}
}

// AND
pub struct NOT;

impl Operator for NOT {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 1);
		match evaluate(&args[0], scope)? {
			Value::Boolean(a) => Ok((!a).into()),
			_ => Err(EggError::OperatorComplaint(NOT_BOOLEAN.to_string())),
		}
	}
}
