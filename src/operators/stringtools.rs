use super::Operator;
use crate::{
	error::{EggError, EggResult},
	evaluator::evaluate,
	expression::{self, Value},
	scope::Scope,
};
use alloc::string::{String, ToString};

pub struct Concat;

impl Operator for Concat {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut Scope) -> EggResult<Value> {
		let mut result = String::with_capacity(args.len() * 64);

		for arg in args {
			match evaluate(arg, scope)? {
				Value::String(string) => result.push_str(&string),
				_ => return Err(EggError::OperatorComplaint("Cannot concatenate non-string".to_string())),
			}
		}

		Ok(Value::String(result.into()))
	}
}

pub struct Length;

impl Operator for Length {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut Scope) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Evaluate
		let res = evaluate(&args[0], scope)?;
		let value = match res {
			Value::String(string) => string.len(),
			_ => return Err(EggError::OperatorComplaint("Cannot get length of non-string".to_string())),
		};

		Ok(Value::Number((value as f32).into()))
	}
}

/// Builtin for slicing and indexing into strings
pub struct Slice;

impl Operator for Slice {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut Scope) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 3);

		// Evaluate
		let res = evaluate(&args[0], scope)?;

		let base = match res {
			Value::String(string) => string,
			_ => return Err(EggError::OperatorComplaint("Cannot slice non-string".to_string())),
		};

		let mut start = match evaluate(&args[1], scope)? {
			Value::Number(num) => num,
			_ => return Err(EggError::OperatorComplaint("Cannot slice with non-number".to_string())),
		};

		(start.0 < 0.0).then(|| start += base.len() as f32);
		let length = match evaluate(&args[2], scope)? {
			Value::Number(num) => num.0 as usize,
			_ => return Err(EggError::OperatorComplaint("Cannot slice with non-number".to_string())),
		};

		let start = start.0 as usize;
		let result = &base[start..start + length];

		Ok(Value::String(result.into()))
	}
}

/// Converts a String to UpperCase
pub struct ToUpper;

impl Operator for ToUpper {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut Scope) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Evaluate
		let res = evaluate(&args[0], scope)?;
		let value = match res {
			Value::String(string) => string.to_uppercase(),
			_ => return Err(EggError::OperatorComplaint("Cannot convert non-string to uppercase".to_string())),
		};

		Ok(Value::String(value.into()))
	}
}

/// Converts a String to Upper case
pub struct ToLower;

impl Operator for ToLower {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut Scope) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Evaluate
		let res = evaluate(&args[0], scope)?;
		let value = match res {
			Value::String(string) => string.to_lowercase(),
			_ => return Err(EggError::OperatorComplaint("Cannot convert non-string to lowercase".to_string())),
		};

		Ok(Value::String(value.into()))
	}
}

/// Trims whitespace from the start and end of a string
pub struct Trim;

impl Operator for Trim {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut Scope) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Evaluate
		let res = evaluate(&args[0], scope)?;
		match res {
			Value::String(string) => Ok(Value::String(string.trim().into())),
			_ => Err(EggError::OperatorComplaint("Cannot trim non-string".to_string())),
		}
	}
}
