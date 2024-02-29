#![allow(clippy::upper_case_acronyms)]

use super::Operator;
use crate::{
	errors::{EggError, EggResult},
	evaluator::evaluate,
	expression::{Expression, Value},
};
use std::{collections::HashMap, rc::Rc};

fn get_resolver() -> &'static mut HashMap<Rc<str>, HashMap<Value, Value>> {
	static mut RESOLVER: Option<HashMap<Rc<str>, HashMap<Value, Value>>> = None;
	unsafe { RESOLVER.get_or_insert(Default::default()) }
}

fn value_into_map_tag(value: Value) -> Result<Rc<str>, EggError> {
	match value {
		Value::String(s) => Ok(s),
		invalid_map_tag => Err(EggError::InvalidMapTag(invalid_map_tag, "Map tag must be a string".into())),
	}
}

/// Creates a new Map and binds it to the specified Value.
pub struct NewMap;

impl Operator for NewMap {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);

		let map_ref = evaluate(&args[0], scope, builtins)?;
		let tag = value_into_map_tag(map_ref)?;

		if get_resolver().contains_key(&tag) {
			return Err(EggError::InvalidMapTag(Value::String(tag), "Map tag already exists".into()));
		}

		get_resolver().insert(tag.clone(), HashMap::new());
		Ok(tag.into())
	}
}

/// Checks if the specified Map exists
pub struct ExistsMap;

impl Operator for ExistsMap {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);
		let tag = evaluate(&args[0], scope, builtins)?;
		let tag = value_into_map_tag(tag)?;
		Ok(get_resolver().contains_key(&tag).into())
	}
}

/// Delete the map at the given map_ref
/// Returns true if the map was deleted, false otherwise
pub struct DeleteMap;

impl Operator for DeleteMap {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);
		let tag = evaluate(&args[0], scope, builtins)?;
		let tag = value_into_map_tag(tag)?;
		Ok(get_resolver().remove(&tag).is_some().into())
	}
}

/// Insert a new value into the specified map
/// API maps to Rust's [`HashMap`]
pub struct Insert;

impl Operator for Insert {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// map_ref, key, value
		assert!(args.len() == 3);

		let tag = evaluate(&args[0], scope, builtins)?;
		let key = evaluate(&args[1], scope, builtins)?;
		let value = evaluate(&args[2], scope, builtins)?;

		let tag = value_into_map_tag(tag)?;
		let res = match get_resolver().get_mut(&tag) {
			Some(map) => map.insert(key, value).unwrap_or(Value::Nil),
			None => return Err(EggError::MapNotFound(tag)),
		};

		Ok(res)
	}
}

/// Print a Map's value to the console
/// API maps to Rust's [`HashMap`]
pub struct PrintMap;

impl Operator for PrintMap {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// map_ref
		assert!(args.len() == 1);

		let tag = evaluate(&args[0], scope, builtins)?;
		let tag = value_into_map_tag(tag)?;

		match get_resolver().get(&tag) {
			Some(map) => println!("{:#?}", map),
			None => return Err(EggError::MapNotFound(tag)),
		};

		Ok(().into())
	}
}

/// Fetch a [Value] the specified map
/// API maps to Rust's [`HashMap`]
pub struct Get;

impl Operator for Get {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// map_ref, key
		assert!(args.len() == 2);

		let tag = evaluate(&args[0], scope, builtins)?;
		let tag = value_into_map_tag(tag)?;
		let key = evaluate(&args[1], scope, builtins)?;

		let res = match get_resolver().get(&tag) {
			Some(map) => map.get(&key),
			None => return Err(EggError::MapNotFound(tag)),
		};

		Ok(res.unwrap_or(&Value::Nil).clone())
	}
}

/// Check if the specified map contains the value
/// API maps to Rust's [`HashMap`]
pub struct Has;

impl Operator for Has {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// map_ref, key
		assert!(args.len() == 2);

		let tag = evaluate(&args[0], scope, builtins)?;
		let tag = value_into_map_tag(tag)?;
		let key = evaluate(&args[1], scope, builtins)?;

		let res = match get_resolver().get(&tag) {
			Some(map) => map.contains_key(&key),
			None => return Err(EggError::MapNotFound(tag)),
		};

		Ok(res.into())
	}
}

/// Delete the given key at the given map
/// API maps to Rust's [`HashMap`]
pub struct Remove;

impl Operator for Remove {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// map_ref, key
		assert!(args.len() == 2);

		let tag = evaluate(&args[0], scope, builtins)?;
		let tag = value_into_map_tag(tag)?;
		let key = evaluate(&args[1], scope, builtins)?;

		let res = match get_resolver().get_mut(&tag) {
			Some(map) => map.remove(&key),
			None => return Err(EggError::MapNotFound(tag)),
		};

		Ok(res.unwrap_or(Value::Nil))
	}
}

/// How many entries does this map have?
/// API maps to Rust's [`HashMap`]
pub struct Size;

impl Operator for Size {
	fn evaluate(&self, args: &[Expression], scope: &mut HashMap<String, Value>, builtins: &HashMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		// map_ref, key
		assert!(args.len() == 1);

		let tag = evaluate(&args[0], scope, builtins)?;
		let tag = value_into_map_tag(tag)?;

		let res = match get_resolver().get(&tag) {
			Some(map) => map.len(),
			None => return Err(EggError::MapNotFound(tag)),
		};

		Ok(Value::Number(res as _))
	}
}
