use super::Operator;
use crate::{
	error::{EggError, EggResult},
	evaluator::evaluate,
	expression::{self, Value},
	scope::Scope,
};
use alloc::{format, string::ToString};

/// Defines a new variable
pub struct Define;

impl Operator for Define {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);
		let name = &args[0];

		match name {
			expression::Expression::Word { name } | expression::Expression::Value { value: Value::String(name) } => {
				let value = evaluate(&args[1], scope)?;
				scope.insert(name.clone(), value)?;
				Ok(Value::Nil)
			}
			_ => Err(EggError::OperatorComplaint("Numbers and Nil cannot be used as variable names".to_string())),
		}
	}
}

/// Mutates an existing variable
pub struct Set;

impl Operator for Set {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);
		let variable_name = &args[0];

		match variable_name {
			expression::Expression::Word { name } => {
				let new_value = evaluate(&args[1], scope)?;
				scope.update(name.clone(), new_value)
			}
			v => {
				return Err(EggError::OperatorComplaint(format!("Non-word variable name. Got: {:?}", v)));
			}
		}

		Ok(Value::Nil)
	}
}

/// Deletes an existing variable
pub struct Delete;

impl Operator for Delete {
	// TODO: Deleting an object should decrease the refcount of any objects stored internally?
	fn evaluate(&self, args: &[expression::Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 1);
		let name = &args[0];

		let res = match name {
			expression::Expression::Word { name } => scope.delete(name.as_str()),
			expression::Expression::Value { value } => match value {
				Value::String(name) => scope.delete(name.as_str()),
				val => return Err(EggError::OperatorComplaint(format!("Cannot delete {val}"))),
			},
			v => return Err(EggError::OperatorComplaint(format!("Cannot delete {v:?}"))),
		};

		Ok(res.unwrap_or(Value::Nil))
	}
}

/// Checks if a variable exists
pub struct Exists;

impl Operator for Exists {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 1);
		let name = &args[0];

		let res = match name {
			expression::Expression::Word { name } => scope.exists(name.as_str()),
			expression::Expression::Value { value } => match value {
				Value::String(name) => scope.exists(name.as_str()),
				val => return Err(EggError::OperatorComplaint(format!("Cannot check if {val} exists"))),
			},
			_ => {
				return Err(EggError::OperatorComplaint("Operations cannot be used as variable names".to_string()));
			}
		};

		Ok(res.into())
	}
}

// Returns the value's type
pub struct TypeOf;

impl super::Operator for TypeOf {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut Scope) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 1);

		let value = evaluate(&args[0], scope)?;

		Ok(match value {
			Value::Number(_) => Value::String(arcstr::literal!("__TYPE__NUMBER")),
			Value::String(_) => Value::String(arcstr::literal!("__TYPE__STRING")),
			Value::Nil => Value::String(arcstr::literal!("__CONSTANT__NIL")),
			Value::Boolean(_) => Value::String(arcstr::literal!("__TYPE__BOOLEAN")),
			Value::Function(_) => Value::String(arcstr::literal!("__TYPE__FUNCTION")),
			Value::Object(_) => Value::String(arcstr::literal!("__TYPE__OBJECT")),
		})
	}
}
