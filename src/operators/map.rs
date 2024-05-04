#![allow(clippy::upper_case_acronyms)]

use super::Operator;
use crate::{
	errors::EggResult,
	evaluator::evaluate,
	expression::{Expression, Value},
	scope::Scope,
};
use alloc::{boxed::Box, collections::BTreeMap};

/// Creates a new Map and binds it to the specified Value.
pub struct NewMap;

impl Operator for NewMap {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);

		let map_ref = evaluate(&args[0], scope, operators)?;
		scope.extras_mut().new_map(map_ref).map(|v| v.into())
	}
}

/// Checks if the specified Map exists
pub struct ExistsMap;

impl Operator for ExistsMap {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);
		let tag = evaluate(&args[0], scope, operators)?;
		scope.extras_mut().contains_map(tag).map(|v| v.into())
	}
}

/// Delete the map at the given map_ref
/// Returns true if the map was deleted, false otherwise
pub struct DeleteMap;

impl Operator for DeleteMap {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);
		let tag = evaluate(&args[0], scope, operators)?;
		scope.extras_mut().delete_map(tag).map(|v| v.into())
	}
}

/// Insert a new value into the specified map
pub struct Insert;

impl Operator for Insert {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 3);

		let tag = evaluate(&args[0], scope, operators)?;
		let key = evaluate(&args[1], scope, operators)?;
		let value = evaluate(&args[2], scope, operators)?;

		scope.extras_mut().insert(tag, key, value).map(|v| v.unwrap_or(Value::Nil))
	}
}

/// Print a Map's value to the console
#[cfg(feature = "std")]
pub struct PrintMap;

#[cfg(feature = "std")]
impl Operator for PrintMap {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);

		let tag = evaluate(&args[0], scope, operators)?;
		scope.extras().print_map(tag)?;

		Ok(().into())
	}
}

/// Fetch a [Value] the specified map
pub struct Get;

impl Operator for Get {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 2);

		let tag = evaluate(&args[0], scope, operators)?;
		let key = evaluate(&args[1], scope, operators)?;

		scope.extras().get(tag, key)
	}
}

/// Check if the specified map contains the key
pub struct Has;

impl Operator for Has {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 2);

		let tag = evaluate(&args[0], scope, operators)?;
		let key = evaluate(&args[1], scope, operators)?;

		scope.extras().has(tag, key).map(|v| v.into())
	}
}

/// Delete the given key at the given map
pub struct Remove;

impl Operator for Remove {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 2);

		let tag = evaluate(&args[0], scope, operators)?;
		let key = evaluate(&args[1], scope, operators)?;

		scope.extras_mut().remove(tag, key).map(|v| v.unwrap_or(Value::Nil))
	}
}

/// How many entries does this map have?
pub struct Size;

impl Operator for Size {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);

		let tag = evaluate(&args[0], scope, operators)?;
		scope.extras().size(tag).map(|v| (v as f32).into())
	}
}

/// Clear the specified map
pub struct Clear;

impl Operator for Clear {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);

		let tag = evaluate(&args[0], scope, operators)?;
		scope.extras_mut().clear(tag)?;

		Ok(().into())
	}
}
