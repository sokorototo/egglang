use alloc::{collections::BTreeMap, format, string::ToString, vec::Vec};
use arcstr::ArcStr;

use crate::{
	error::{EggError, EggResult},
	evaluator::evaluate,
	expression::{Expression, Value},
	operators::Operator,
};

use super::Scope;

/// A function defined withing the Egg script.
///
/// Functions are defined using the `fn` operator.
/// They keep track of their argument names. These are later used to construct a local [`Scope`] during function invocation.
/// The body of the function is an [`Expression`], which is [`evaluated`](evaluate) using the newly created local [`Scope`].
pub struct FunctionDefinition {
	/// The names of the parameters of the function.
	pub parameter_names: Vec<ArcStr>,
	/// The body of the function.
	pub body: Expression,
}

impl core::fmt::Debug for FunctionDefinition {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let parameters = self.parameter_names.first().map(|s| s.as_str()).unwrap_or("");
		let parameters = self.parameter_names.iter().skip(1).fold(parameters.to_string(), |acc, s| format!("{}, {}", acc, s.as_str()));

		write!(f, "Function ({})", parameters)
	}
}

fn get_parameter_name(expr: &Expression) -> EggResult<ArcStr> {
	match expr {
		Expression::Word { name } => Ok(name.clone()),
		_ => Err(EggError::InvalidFunctionDefinition("Parameter name must be a word".to_string())),
	}
}

impl super::Scope {
	pub(crate) fn get_function(&self, name: &str) -> Option<usize> {
		let value = self.get(name);
		match value {
			Some(Value::Function(idx)) => Some(*idx),
			_ => None,
		}
	}

	pub fn get_function_definition(&self, idx: usize) -> EggResult<&FunctionDefinition> {
		self.extras()
			.functions
			.get(&idx)
			.ok_or_else(|| EggError::InvalidFunctionCall(format!("Function with index {} not found", idx)))
	}

	pub fn get_function_definition_mut(&mut self, idx: usize) -> EggResult<&mut FunctionDefinition> {
		self.extras_mut()
			.functions
			.get_mut(&idx)
			.ok_or_else(|| EggError::InvalidFunctionCall(format!("Function with index {} not found", idx)))
	}

	pub fn call_function(&mut self, idx: usize, parameters: &[Expression]) -> EggResult<Value> {
		let function = unsafe {
			// SAFETY: We are not modifying the scope, only reading the function definition from it
			(*(self as *const Scope)).get_function_definition(idx)?
		};

		if parameters.len() != function.parameter_names.len() {
			return Err(EggError::InvalidFunctionCall(format!(
				"Function expects {} parameters, but {} were given",
				function.parameter_names.len(),
				parameters.len()
			)));
		}

		let mut new_scope = BTreeMap::new();
		for (name, expression) in function.parameter_names.iter().zip(parameters.iter()) {
			let value = evaluate(expression, self)?;
			new_scope.insert(name.clone(), value);
		}

		let mut local_scope = self.local(new_scope);
		evaluate(&function.body, &mut local_scope)
	}

	pub fn delete_function(&mut self, idx: usize) -> Option<FunctionDefinition> {
		self.extras_mut().functions.remove(&idx)
	}
}

/// Create a new Function
pub struct CreateFunction;

impl Operator for CreateFunction {
	fn evaluate(&self, args: &[Expression], scope: &mut super::Scope) -> EggResult<crate::expression::Value> {
		if args.is_empty() {
			return Err(EggError::InvalidFunctionDefinition("Function Definition requires at least a body".to_string()));
		}

		// assemble function parts
		let body = args[args.len() - 1].clone();
		let parameter_names = args.iter().take(args.len() - 1).map(get_parameter_name).collect::<EggResult<Vec<ArcStr>>>()?;

		scope.extras_mut().counter += 1;
		let index = scope.extras().counter;
		scope.extras_mut().functions.insert(index, FunctionDefinition { parameter_names, body });

		Ok(crate::expression::Value::Function(index))
	}
}
