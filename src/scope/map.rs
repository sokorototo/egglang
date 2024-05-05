use crate::{
	errors::EggError,
	errors::EggResult,
	evaluator::evaluate,
	expression::{Expression, Value},
	operators::Operator,
	scope::Scope,
};
use alloc::{boxed::Box, collections::BTreeMap};
use arcstr::ArcStr;

fn validate_map_tag(value: Value) -> EggResult<ArcStr> {
	match value {
		Value::String(s) => Ok(s),
		i => Err(EggError::InvalidMapTag(i, "Map tag must be a string".into())),
	}
}

impl Scope {
	pub fn contains_map(&self, tag: Value) -> EggResult<bool> {
		let tag = validate_map_tag(tag)?;
		Ok(self.extras().maps.contains_key(&tag))
	}

	pub fn new_map(&mut self, tag: Value) -> EggResult<ArcStr> {
		let tag = validate_map_tag(tag)?;

		if self.extras().maps.contains_key(&tag) {
			Err(EggError::InvalidMapTag(tag.into(), "Map tag already exists".into()))
		} else {
			self.extras_mut().maps.insert(tag.clone(), BTreeMap::new());
			Ok(tag)
		}
	}

	#[cfg(feature = "std")]
	pub fn print_map(&self, tag: Value) -> EggResult<()> {
		let tag = validate_map_tag(tag)?;
		let map = self.extras().maps.get(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		println!("{:#?}", map);
		Ok(())
	}

	pub fn delete_map(&mut self, tag: Value) -> EggResult<bool> {
		let tag = validate_map_tag(tag)?;
		Ok(self.extras_mut().maps.remove(&tag).is_some())
	}

	pub fn map_get(&self, map_tag: Value, key: Value) -> EggResult<Value> {
		let tag = validate_map_tag(map_tag)?;
		let map = self.extras().maps.get(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		Ok(map.get(&key).cloned().unwrap_or(Value::Nil))
	}

	pub fn map_insert(&mut self, map_tag: Value, key: Value, value: Value) -> EggResult<Option<Value>> {
		let tag = validate_map_tag(map_tag)?;
		let map = self.extras_mut().maps.get_mut(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		Ok(map.insert(key, value))
	}

	pub fn map_has(&self, map_tag: Value, key: Value) -> EggResult<bool> {
		let tag = validate_map_tag(map_tag)?;
		let map = self.extras().maps.get(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		Ok(map.contains_key(&key))
	}

	pub fn map_remove(&mut self, map_tag: Value, key: Value) -> EggResult<Option<Value>> {
		let tag = validate_map_tag(map_tag)?;
		let map = self.extras_mut().maps.get_mut(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		Ok(map.remove(&key))
	}

	pub fn map_clear(&mut self, map_tag: Value) -> EggResult<()> {
		let tag = validate_map_tag(map_tag)?;
		let map = self.extras_mut().maps.get_mut(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		map.clear();
		Ok(())
	}

	pub fn map_size(&self, map_tag: Value) -> EggResult<usize> {
		let tag = validate_map_tag(map_tag)?;
		let map = self.extras().maps.get(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		Ok(map.len())
	}
}

/// Creates a new Map and binds it to the specified Value.
pub struct NewMap;

impl Operator for NewMap {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);

		let map_ref = evaluate(&args[0], scope, operators)?;
		scope.new_map(map_ref).map(|v| v.into())
	}
}

/// Checks if the specified Map exists
pub struct ExistsMap;

impl Operator for ExistsMap {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);
		let tag = evaluate(&args[0], scope, operators)?;
		scope.contains_map(tag).map(|v| v.into())
	}
}

/// Delete the map at the given map_ref
/// Returns true if the map was deleted, false otherwise
pub struct DeleteMap;

impl Operator for DeleteMap {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);
		let tag = evaluate(&args[0], scope, operators)?;
		scope.delete_map(tag).map(|v| v.into())
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
		scope.print_map(tag)?;

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

		scope.map_remove(tag, key).map(|v| v.unwrap_or(Value::Nil))
	}
}

/// How many entries does this map have?
pub struct Size;

impl Operator for Size {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);

		let tag = evaluate(&args[0], scope, operators)?;
		scope.map_size(tag).map(|v| (v as f32).into())
	}
}

/// Clear the specified map
pub struct Clear;

impl Operator for Clear {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
		assert!(args.len() == 1);

		let tag = evaluate(&args[0], scope, operators)?;
		scope.map_clear(tag)?;

		Ok(().into())
	}
}
