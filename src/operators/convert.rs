use super::Operator;
use crate::{
	error::{EggError, EggResult},
	evaluator::evaluate,
	expression::{self, Value},
	scope::Scope,
};
use alloc::string::ToString as _;

// Operation that converts numbers to strings
pub struct ToString;

impl Operator for ToString {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut Scope) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Evaluate
		let res = evaluate(&args[0], scope)?;
		Ok(Value::String(res.to_string().into()))
	}
}

// Operation that converts strings to numbers
pub struct ToNumber;

impl Operator for ToNumber {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut Scope) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Evaluate
		match evaluate(&args[0], scope)? {
			Value::String(string) => string.parse::<f32>().map(|f| f.into()).map_err(|e| EggError::OperatorComplaint(e.to_string())),
			Value::Boolean(b) => Ok((if b { 0.0 } else { 1.0 }).into()),
			Value::Nil => Err(EggError::OperatorComplaint("Can't convert Nil to a number".to_string())),
			Value::Function(_) => Err(EggError::OperatorComplaint("Can't convert a Function to a number".to_string())),
			Value::Object(_) => Err(EggError::OperatorComplaint("Can't convert an Object to a number".to_string())),
			n => Ok(n),
		}
	}
}
