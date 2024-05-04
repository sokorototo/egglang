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

// Evaluates all expressions defined within it's operands
pub struct Do;

impl Operator for Do {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		args.iter().try_fold(Value::Nil, |_, nxt| evaluate(nxt, scope, builtins))
	}
}

// Simple if statement
pub struct If;

impl Operator for If {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 3);

		// Evaluate
		let condition = evaluate(&args[0], scope, builtins)?;
		let value = match condition {
			Value::Number(num) => num != 0.0,
			Value::Boolean(b) => b,
			#[rustfmt::skip]
            _ => return Err(EggError::OperatorComplaint("if(--) expects a boolean (a number that if zero equals false) as it's parameter".to_string())),
		};

		if value {
			evaluate(&args[1], scope, builtins)
		} else {
			evaluate(&args[2], scope, builtins)
		}
	}
}

// Simple while loop
pub struct While;

impl Operator for While {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 2);

		// Loop
		let mut loop_result = Value::Nil;

		loop {
			let condition = evaluate(&args[0], scope, builtins)?;

			let continue_condition = match condition {
				Value::Number(num) => num != 0.0,
				Value::Boolean(b) => b,
				#[rustfmt::skip]
                _ => return Err(EggError::OperatorComplaint("while(--) expects a number as it's parameter".to_string())),
			};

			if !continue_condition {
				break Ok(loop_result);
			}

			// Evaluate expression
			loop_result = evaluate(&args[1], scope, builtins)?;
		}
	}
}

// Simpler loop construct
pub struct Repeat;

impl Operator for Repeat {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 2);

		// Loop
		let mut iterations = 0.0;
		let mut loop_value = Value::Nil;

		let max_iter = match evaluate(&args[0], scope, builtins)? {
			Value::Number(num) => num,
			#[rustfmt::skip]
            _ => return Err(EggError::OperatorComplaint("repeat(--, ...) expects a number as it's first parameter".to_string())),
		};

		loop {
			// Repeat X times
			if iterations >= max_iter.0 {
				break Ok(loop_value);
			}

			// Evaluate expression
			loop_value = evaluate(&args[1], scope, builtins)?;

			iterations += 1.0;
		}
	}
}

// Sleep for x milliseconds
#[cfg(feature = "std")]
pub struct Sleep;

#[cfg(feature = "std")]
impl Operator for Sleep {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		use std::{thread::sleep, time::Duration};

		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Loop
		let sleep_time = evaluate(&args[0], scope, builtins)?;
		if let Value::Number(value) = sleep_time {
			let duration = Duration::from_millis(value as u64);
			sleep(duration)
		} else {
			#[rustfmt::skip]
            return Err(EggError::OperatorComplaint("sleep(--) expects a number as it's parameter".to_string()));
		}

		Ok(sleep_time)
	}
}

// Sleep for ∞ milliseconds
pub struct Panic;

impl Operator for Panic {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 1);

		// Loop
		match evaluate(&args[0], scope, builtins)? {
			Value::Number(error_code) => {
				panic!("Program has met an unexpected error: ErrorCode: {error_code}")
			}
			Value::String(message) => panic!("{message}"),
			_ => panic!("Program has terminated prematurely due to an unexpected error"),
		}
	}
}

pub struct Assert;

impl Operator for Assert {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// Assert correct length of arguments
		debug_assert_eq!(args.len(), 2);

		let message = match &evaluate(&args[0], scope, builtins)? {
			Value::Boolean(b) if !b => Some(evaluate(&args[1], scope, builtins)?),
			_ => None,
		};

		if let Some(message) = message {
			return Err(EggError::AssertionFailed(message));
		}

		Ok(Value::Nil)
	}
}
