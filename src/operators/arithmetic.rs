use super::Operator;
use crate::{
	errors::{EggError, EggResult},
	evaluator::evaluate,
	expression::{Expression, Value},
};
use std::collections::HashMap;

// Basic add operation
pub struct Add;

impl Operator for Add {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		args.iter()
			.map(|arg| evaluate(arg, scope, builtins))
			.map(|d| match d {
				Ok(Value::Number(num)) => Ok(num),
				Ok(v) => Err(EggError::OperatorComplaint(format!("Invalid argument: {v}, please provide a number"))),
				Err(e) => Err(e),
			})
			.try_fold(0, |acc, d| d.map(|d| acc + d))
			.map(Value::Number)
	}
}

// Basic multiply operation
pub struct Multiply;

impl Operator for Multiply {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		args.iter()
			.map(|arg| evaluate(arg, scope, builtins))
			.map(|d| match d {
				Ok(Value::Number(num)) => Ok(num),
				Ok(v) => Err(EggError::OperatorComplaint(format!("Invalid argument: {v}, please provide a number"))),
				Err(e) => Err(e),
			})
			.try_fold(0, |acc, d| d.map(|d| acc * d))
			.map(Value::Number)
	}
}

// Basic minus operation
pub struct Subtract;

impl Operator for Subtract {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope, builtins)?;
		let val2 = evaluate(&args[1], scope, builtins)?;

		match (val1, val2) {
			(Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
			(a, b) => Err(EggError::OperatorComplaint(format!("Invalid arguments: {a}, {b} please provide a numbers"))),
		}
	}
}

// Basic divide operation
pub struct Divide;

impl Operator for Divide {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);

		let val1 = evaluate(&args[0], scope, builtins)?;
		let val2 = evaluate(&args[1], scope, builtins)?;

		match (val1, val2) {
			(Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
			(a, b) => Err(EggError::OperatorComplaint(format!("Invalid argument: {a}, {b} please provide a numbers"))),
		}
	}
}
