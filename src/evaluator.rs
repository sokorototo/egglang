use crate::{
	errors::{EggError, EggResult},
	expression::{Expression, Value},
	operators::Operator,
	scope::Scope,
};
use alloc::{boxed::Box, collections::BTreeMap};

pub static mut EVALUATIONS: u64 = 0;

/// Given an expression, evaluate it and return the result
pub fn evaluate(expr: &Expression, scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
	unsafe { EVALUATIONS += 1 };

	match expr {
		Expression::Value { value } => Ok(value.clone()),
		Expression::Word { name } => scope.get(name.as_str()).ok_or_else(|| EggError::UndefinedBinding(name.clone())).map(|d| d.clone()),
		Expression::FnCall { name, parameters } => {
			// Search for a user-defined function in the scope
			if let Some(function) = scope.get_function(name) {
				scope.call_function(function, parameters, operators)
			} else {
				// Search for a built-in operator with the given name
				let operator = operators.get(name.as_str()).ok_or_else(|| EggError::FunctionNotFound(name.clone()))?;
				operator.evaluate(parameters, scope, operators)
			}
		}
	}
}
