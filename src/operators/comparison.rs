use crate::{
	error::{EggError, EggResult},
	evaluator::evaluate,
	expression::{Expression, Value},
	scope::Scope,
};
use alloc::string::ToString;

// Checks for equality
pub struct Equals;

impl super::Operator for Equals {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope)?;
		let val2 = evaluate(&args[1], scope)?;

		Ok((val1 == val2).into())
	}
}

// Checks for inequality
pub struct NotEquals;

impl super::Operator for NotEquals {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope)?;
		let val2 = evaluate(&args[1], scope)?;

		Ok((val1 != val2).into())
	}
}

// Greater than
pub struct GreaterThan;

impl super::Operator for GreaterThan {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope)?;
		let val2 = evaluate(&args[1], scope)?;

		match (val1, val2) {
			(Value::Number(a), Value::Number(b)) => Ok((a > b).into()),
			_ => Err(EggError::OperatorComplaint("please provide numbers as arguments for mathematical operations".to_string())),
		}
	}
}

/// Lesser than
pub struct LessThan;

impl super::Operator for LessThan {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope)?;
		let val2 = evaluate(&args[1], scope)?;

		match (val1, val2) {
			(Value::Number(a), Value::Number(b)) => Ok((a < b).into()),
			_ => Err(EggError::OperatorComplaint("please provide numbers as arguments for mathematical operations".to_string())),
		}
	}
}

/// Checks if the value is nil
pub struct IsNil;

impl super::Operator for IsNil {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 1);

		Ok(matches!(evaluate(&args[0], scope)?, Value::Nil).into())
	}
}
