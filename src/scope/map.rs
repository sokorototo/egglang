use crate::{
	errors::EggError,
	errors::EggResult,
	evaluator::evaluate,
	expression::{Expression, Value},
	operators::Operator,
	scope::Scope,
};
use alloc::{boxed::Box, collections::BTreeMap};

fn validate_map_tag(value: Value) -> EggResult<usize> {
	match value {
		Value::Object(s) => Ok(s),
		i => Err(EggError::InvalidMapTag(i, "Map tag must be a string".into())),
	}
}

impl Scope {
	pub fn get_map(&self, tag: usize) -> EggResult<&BTreeMap<Value, Value>> {
		self.extras().maps.get(&tag).ok_or_else(|| EggError::MapNotFound(tag))
	}

	pub fn get_map_mut(&mut self, tag: usize) -> EggResult<&mut BTreeMap<Value, Value>> {
		self.extras_mut().maps.get_mut(&tag).ok_or_else(|| EggError::MapNotFound(tag))
	}

	pub fn get_map_tag(&self, tag: Value) -> EggResult<usize> {
		let tag = validate_map_tag(tag)?;
		if self.extras().maps.contains_key(&tag) {
			Ok(tag)
		} else {
			Err(EggError::MapNotFound(tag))
		}
	}

	pub fn new_map(&mut self) -> EggResult<Value> {
		self.extras_mut().current_map_index += 1;
		let index = self.extras().current_map_index;

		self.extras_mut().maps.insert(index, BTreeMap::new());
		Ok(Value::Object(index))
	}

	#[cfg(feature = "std")]
	pub fn print_map(&self, tag: usize) -> EggResult<()> {
		self.get_map(tag).map(|m| println!("Map: {:?}", m))
	}

	pub fn delete_map(&mut self, tag: usize) -> bool {
		self.extras_mut().maps.remove(&tag).is_some()
	}

	pub fn map_get(&self, tag: usize, key: Value) -> EggResult<Value> {
		self.get_map(tag).map(|m| m.get(&key).cloned().unwrap_or(Value::Nil))
	}

	pub fn map_insert(&mut self, tag: usize, key: Value, value: Value) -> EggResult<Option<Value>> {
		self.get_map_mut(tag).map(|m| m.insert(key, value))
	}

	pub fn map_has(&self, tag: usize, key: Value) -> EggResult<bool> {
		self.get_map(tag).map(|m| m.contains_key(&key))
	}

	pub fn map_remove(&mut self, tag: usize, key: Value) -> EggResult<Option<Value>> {
		self.get_map_mut(tag).map(|m| m.remove(&key))
	}

	pub fn map_clear(&mut self, tag: usize) -> EggResult<()> {
		self.get_map_mut(tag).map(|m| m.clear())
	}

	pub fn map_size(&self, tag: usize) -> EggResult<usize> {
		self.get_map(tag).map(|m| m.len())
	}
}

/// Creates a new Map and binds it to the specified Value.
pub struct NewMap;

impl Operator for NewMap {
	fn evaluate(&self, _: &[Expression], scope: &mut Scope, _: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		scope.new_map()
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

		let tag = scope.get_map_tag(tag)?;
		scope.map_insert(tag, key, value).map(|v| v.unwrap_or(Value::Nil))
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
		let tag = scope.get_map_tag(tag)?;
		scope.print_map(tag).map(|_| Value::Nil)
	}
}

/// Fetch a [Value] the specified map
pub struct Get;

impl Operator for Get {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 2);

		let tag = evaluate(&args[0], scope, operators)?;
		let key = evaluate(&args[1], scope, operators)?;

		let tag = scope.get_map_tag(tag)?;
		scope.map_get(tag, key)
	}
}

/// Check if the specified map contains the key
pub struct Has;

impl Operator for Has {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 2);

		let tag = evaluate(&args[0], scope, operators)?;
		let key = evaluate(&args[1], scope, operators)?;

		let tag = scope.get_map_tag(tag)?;
		scope.map_has(tag, key).map(|v| v.into())
	}
}

/// Delete the given key at the given map
pub struct Remove;

impl Operator for Remove {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 2);

		let tag = evaluate(&args[0], scope, operators)?;
		let key = evaluate(&args[1], scope, operators)?;

		let tag = scope.get_map_tag(tag)?;
		scope.map_remove(tag, key).map(|v| v.unwrap_or(Value::Nil))
	}
}

/// How many entries does this map have?
pub struct Size;

impl Operator for Size {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);

		let tag = evaluate(&args[0], scope, operators)?;
		let tag = scope.get_map_tag(tag)?;

		scope.map_size(tag).map(|v| (v as f32).into())
	}
}

/// Clear the specified map
pub struct Clear;

impl Operator for Clear {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);

		let tag = evaluate(&args[0], scope, operators)?;
		let tag = scope.get_map_tag(tag)?;

		scope.map_clear(tag).map(|_| Value::Nil)
	}
}
