use crate::{
	error::{EggError, EggResult},
	expression::{Expression, Value},
	operators::Operator,
	scope::Scope,
};
use alloc::{boxed::Box, collections::BTreeMap};

/// Counts calls to the [`evaluate`] function. Used as a statistic, thus object safety is not of
pub static mut EVALUATIONS: u64 = 0;

/// Given an [`Expression`], evaluate and yield a [`Value`].
/// Requires that the user assemble a [`Scope`] and a map of [`Operator`]s.
///
/// ```
/// use egglang::prelude::*;
///
/// // Create the default Scope, with necessary constants set
/// let mut scope = Scope::default();
///
/// // Create a minimal set of operators
/// let mut operators = operators::empty();
/// operators::minimal(&mut operators);
///
/// // Parse a Script into a list of expressions
/// let expression = parse("sum(1 multiply(2 5))").unwrap();
///
/// // Evaluate the expression
/// let result = evaluate(&expression[0], &mut scope, &operators).unwrap();
/// assert_eq!(result, 11.0.into());
/// ```
pub fn evaluate(expr: &Expression, scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
	unsafe { EVALUATIONS += 1 };

	match expr {
		Expression::Value { value } => Ok(value.clone()),
		Expression::Word { name } => scope.get(name.as_str()).ok_or_else(|| EggError::UndefinedBinding(name.clone())).cloned(),
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
