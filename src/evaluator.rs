use crate::{
	error::{EggError, EggResult},
	expression::{Expression, Function, Value},
	scope::Scope,
};

/// Increments for each call to [`evaluate`], including internal calls
pub static mut EVALUATIONS: u128 = 0;

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
pub fn evaluate(expr: &Expression, scope: &mut Scope) -> EggResult<Value> {
	unsafe { EVALUATIONS += 1 };

	match expr {
		Expression::Value { value } => Ok(value.clone()),
		Expression::Word { name } => scope.get(name.as_str()).ok_or_else(|| EggError::UndefinedBinding(name.clone())).cloned(),
		Expression::FnCall { function: identifier, parameters } => match identifier {
			Function::Script(name) => {
				let idx = scope.get_function(name).ok_or_else(|| EggError::FunctionNotFound(name.clone()))?;
				scope.call_function(idx, parameters)
			}
			Function::Host(op) => {
				let op = unsafe { op.as_ref().unwrap_unchecked() };
				op.evaluate(parameters, scope)
			}
		},
	}
}
