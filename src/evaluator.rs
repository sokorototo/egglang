use crate::{
	errors::{EggError, EggResult},
	expression::{Expression, Value},
	operators::Operator,
};
use std::collections::HashMap;

pub static mut EVALUATIONS: u64 = 0;

/// Given an expression, evaluate it and return the result
pub fn evaluate(expr: &Expression, scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
	unsafe { EVALUATIONS += 1 };

	match expr {
		Expression::Value { value } => Ok(value.clone()),
		Expression::Word { name } => scope.get(name.as_ref()).ok_or_else(|| EggError::UndefinedBinding(name.to_string())).map(|d| d.clone()),
		Expression::Operation { name, parameters } => {
			// Get operation's name
			let name = name.as_ref();

			// Fetch operation
			let operator = builtins.get(name).ok_or_else(|| EggError::SpecialFormNotFound(name.to_string()))?;

			operator.evaluate(parameters, scope, builtins)
		}
	}
}
