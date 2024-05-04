use crate::{
	errors::{EggError, EggResult},
	expression::{Expression, Value},
	operators::Operator,
};
use alloc::{boxed::Box, collections::BTreeMap, string::String};

pub static mut EVALUATIONS: u64 = 0;

/// Given an expression, evaluate it and return the result
pub fn evaluate(expr: &Expression, scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
	unsafe { EVALUATIONS += 1 };

	match expr {
		Expression::Value { value } => Ok(value.clone()),
		Expression::Word { name } => scope.get(name.as_str()).ok_or_else(|| EggError::UndefinedBinding(name.to_string())).map(|d| d.clone()),
		Expression::Operation { name, parameters } => {
			// Fetch operation
			let operator = builtins.get(name.as_str()).ok_or_else(|| EggError::SpecialFormNotFound(name.clone()))?;
			operator.evaluate(parameters, scope, builtins)
		}
	}
}
