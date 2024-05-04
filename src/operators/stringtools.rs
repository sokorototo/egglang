use super::Operator;
use crate::{
	errors::{EggError, EggResult},
	evaluator::evaluate,
	expression::{self, Value},
};
use alloc::{
	boxed::Box,
	collections::BTreeMap,
	string::{String, ToString},
};

pub struct Concat;

impl Operator for Concat {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		let mut result = String::with_capacity(args.len() * 64);

		for arg in args {
			match evaluate(arg, scope, operators)? {
				Value::String(string) => result.push_str(&string),
				#[rustfmt::skip]
                _ => return Err(EggError::OperatorComplaint("Cannot concatenate non-string".to_string())),
			}
		}

		Ok(Value::String(result.into()))
	}
}

pub struct Length;

impl Operator for Length {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Evaluate
		let res = evaluate(&args[0], scope, operators)?;
		let value = match res {
			Value::String(string) => string.len(),
			#[rustfmt::skip]
            _ => return Err(EggError::OperatorComplaint("Cannot get length of non-string".to_string())),
		};

		Ok(Value::Number((value as f32).into()))
	}
}

// Define a special form that extracts a slice from a string and produces a new string given a start and a length
pub struct Slice;

impl Operator for Slice {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 3);

		// Evaluate
		let res = evaluate(&args[0], scope, operators)?;

		let base = match res {
			Value::String(string) => string,
			#[rustfmt::skip]
            _ => return Err(EggError::OperatorComplaint("Cannot slice non-string".to_string())),
		};

		let mut start = match evaluate(&args[1], scope, operators)? {
			Value::Number(num) => num,
			#[rustfmt::skip]
            _ => return Err(EggError::OperatorComplaint("Cannot slice with non-number".to_string())),
		};

		(start.0 < 0.0).then(|| start += base.len() as f32);
		let length = match evaluate(&args[2], scope, operators)? {
			Value::Number(num) => num.0 as usize,
			#[rustfmt::skip]
            _ => return Err(EggError::OperatorComplaint("Cannot slice with non-number".to_string())),
		};

		let start = start.0 as usize;
		let result = &base[start..start + length];

		Ok(Value::String(result.into()))
	}
}

// Define two special forms that take a string and convert to uppercase and lowercase respectively
pub struct ToUpper;

impl Operator for ToUpper {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Evaluate
		let res = evaluate(&args[0], scope, operators)?;
		let value = match res {
			Value::String(string) => string.to_uppercase(),
			#[rustfmt::skip]
            _ => return Err(EggError::OperatorComplaint("Cannot convert non-string to uppercase".to_string())),
		};

		Ok(Value::String(value.into()))
	}
}

pub struct ToLower;

impl Operator for ToLower {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Evaluate
		let res = evaluate(&args[0], scope, operators)?;
		let value = match res {
			Value::String(string) => string.to_lowercase(),
			#[rustfmt::skip]
            _ => return Err(EggError::OperatorComplaint("Cannot convert non-string to lowercase".to_string())),
		};

		Ok(Value::String(value.into()))
	}
}

pub struct Trim;

impl Operator for Trim {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Evaluate
		let res = evaluate(&args[0], scope, operators)?;
		match res {
			Value::String(string) => Ok(Value::String(string.trim().into())),
			#[rustfmt::skip]
            _ => Err(EggError::OperatorComplaint("Cannot trim non-string".to_string())),
		}
	}
}
