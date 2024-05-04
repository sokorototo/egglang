use super::Operator;
use crate::{
	errors::{EggError, EggResult},
	evaluator::evaluate,
	expression::{self, Value},
};
use alloc::{
	boxed::Box,
	collections::BTreeMap,
	format,
	string::{String, ToString},
};

/// Defines a new variable
pub struct Define;

impl Operator for Define {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);
		let name = &args[0];

		match name {
			expression::Expression::Word { name } => {
				let value = evaluate(&args[1], scope, builtins)?;

				if scope.contains_key(name.as_str()) {
					#[rustfmt::skip]
                    return Err(EggError::OperatorComplaint(format!( "variable {} already defined", name )));
				} else {
					// THIS IS BASICALLY A CLONE
					scope.insert(name.to_string(), value.clone());
				}

				Ok(value)
			}
			expression::Expression::Value { value: Value::String(name) } => {
				let value = evaluate(&args[1], scope, builtins)?;

				if scope.contains_key(name.as_str()) {
					#[rustfmt::skip]
                    return Err(EggError::OperatorComplaint(format!( "variable {} already defined", name )));
				} else {
					scope.insert(name.to_string(), value.clone());
				}

				Ok(value)
			}
			_ => Err(EggError::OperatorComplaint("Numbers and Nil cannot be used as variable names".to_string())),
		}
	}
}

/// Mutates an existing variable
pub struct Set;

impl Operator for Set {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 2);
		let variable_name = &args[0];
		let old_value;

		match variable_name {
			expression::Expression::Word { name } => {
				let new_value = evaluate(&args[1], scope, builtins)?;
				old_value = evaluate(variable_name, scope, builtins);

				scope.get_mut(name.as_str()).map(|val| *val = new_value);
			}
			_ => {
				return Err(EggError::OperatorComplaint("Numbers and Nil cannot be used as variable names".to_string()));
			}
		}

		old_value
	}
}

/// Deletes an existing variable
pub struct Delete;

impl Operator for Delete {
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, _: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 1);
		let name = &args[0];

		let res = match name {
			expression::Expression::Word { name } => scope.remove(name.as_str()),
			expression::Expression::Value { value } => match value {
				Value::String(name) => scope.remove(name.as_str()),
				#[rustfmt::skip]
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
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, _: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 1);
		let name = &args[0];

		let res = match name {
			expression::Expression::Word { name } => scope.contains_key(name.as_str()),
			expression::Expression::Value { value } => match value {
				Value::String(name) => scope.contains_key(name.as_str()),
				#[rustfmt::skip]
                val => return Err(EggError::OperatorComplaint(format!( "Cannot check if {val} exists" ))),
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
	fn evaluate(&self, args: &[expression::Expression], scope: &mut BTreeMap<String, Value>, builtins: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		debug_assert_eq!(args.len(), 1);

		let value = evaluate(&args[0], scope, builtins)?;

		Ok(match value {
			Value::Number(_) => Value::String("__NUMBER".into()),
			Value::String(_) => Value::String("__STRING".into()),
			Value::Nil => Value::String("__NIL".into()),
			Value::Boolean(_) => Value::String("__BOOLEAN".into()),
		})
	}
}
