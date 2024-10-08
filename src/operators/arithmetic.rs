use super::Operator;
use crate::{
	error::{EggError, EggResult},
	evaluator::evaluate,
	expression::{Expression, Value},
	scope::Scope,
};
use alloc::format;

// Basic add operation
pub struct Sum;

impl Operator for Sum {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		args.iter()
			.map(|arg| evaluate(arg, scope))
			.map(|d| match d {
				Ok(Value::Number(num)) => Ok(num),
				Ok(v) => Err(EggError::OperatorComplaint(format!("Invalid argument: {v}, please provide a number"))),
				Err(e) => Err(e),
			})
			.try_fold(0.0, |acc, d| d.map(|d| acc + d.0))
			.map(|f| f.into())
	}
}

// Basic multiply operation
pub struct Multiply;

impl Operator for Multiply {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		args.iter()
			.map(|arg| evaluate(arg, scope))
			.map(|d| match d {
				Ok(Value::Number(num)) => Ok(num),
				Ok(v) => Err(EggError::OperatorComplaint(format!("Invalid argument: {v}, please provide a number"))),
				Err(e) => Err(e),
			})
			.try_fold(1.0, |acc, d| d.map(|d| acc * d.0))
			.map(|f| f.into())
	}
}

// Basic minus operation
pub struct Subtract;

impl Operator for Subtract {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope)?;
		let val2 = evaluate(&args[1], scope)?;

		match (val1, val2) {
			(Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
			(a, b) => Err(EggError::OperatorComplaint(format!("Arguments: {a}, {b} must both be numbers"))),
		}
	}
}

// Basic divide operation
pub struct Divide;

impl Operator for Divide {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope)?;
		let val2 = evaluate(&args[1], scope)?;

		match (val1, val2) {
			(Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
			(a, b) => Err(EggError::OperatorComplaint(format!("Arguments: {a}, {b} must both be numbers"))),
		}
	}
}

// Basic modulus operation
pub struct Modulus;

impl Operator for Modulus {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope)?;
		let val2 = evaluate(&args[1], scope)?;

		match (val1, val2) {
			(Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
			(a, b) => Err(EggError::OperatorComplaint(format!("Arguments: {a}, {b} must both be numbers"))),
		}
	}
}
