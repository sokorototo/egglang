use alloc::collections::BTreeMap;

use crate::{
	error::EggError,
	error::EggResult,
	evaluator::evaluate,
	expression::{Expression, Value},
	operators::Operator,
	scope::Scope,
};

impl Scope {
	pub fn create_object(&mut self) -> EggResult<Value> {
		self.extras_mut().counter += 1;
		let index = self.extras().counter;

		self.extras_mut().maps.insert(index, BTreeMap::new());
		Ok(Value::Object(index))
	}

	pub fn get_object_tag(&self, tag: Value) -> EggResult<usize> {
		let Value::Object(idx) = &tag else { return Err(EggError::InvalidObjectReference(tag)) };

		match self.extras().maps.contains_key(idx) {
			true => Ok(idx.clone()),
			false => Err(EggError::InvalidObjectReference(tag)),
		}
	}

	#[inline]
	pub fn get_object(&self, tag: usize) -> &BTreeMap<Value, Value> {
		self.extras().maps.get(&tag).expect("Object Not Found")
	}

	#[inline]
	pub fn get_object_mut(&mut self, tag: usize) -> &mut BTreeMap<Value, Value> {
		self.extras_mut().maps.get_mut(&tag).expect("Object Not Found")
	}

	#[inline]
	pub fn delete_object(&mut self, tag: usize) -> Option<BTreeMap<Value, Value>> {
		self.extras_mut().maps.remove(&tag)
	}
}

/// Creates a new Object and binds it to the specified [`Value`]
pub struct CreateObject;

impl Operator for CreateObject {
	fn evaluate(&self, _: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		scope.create_object()
	}
}

/// Insert a new value into the specified map
pub struct Insert;

impl Operator for Insert {
	// TODO: Now we have to think about references, lifetimes and garbage collection
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		assert!(args.len() == 3);

		let tag = evaluate(&args[0], scope)?;

		// keys can only be primitives
		let key = evaluate(&args[1], scope)?;
		if !key.is_primitive() {
			return Err(EggError::InvalidObjectKey(key));
		}

		let value = evaluate(&args[2], scope)?;

		let tag = scope.get_object_tag(tag)?;
		let map = scope.get_object_mut(tag);

		Ok(map.insert(key, value).into())
	}
}

/// Fetch a [Value] the specified map
pub struct Get;

impl Operator for Get {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		assert!(args.len() == 2);

		let tag = evaluate(&args[0], scope)?;
		let key = evaluate(&args[1], scope)?;

		let tag = scope.get_object_tag(tag)?;
		let map = scope.get_object(tag);

		Ok(map.get(&key).cloned().into())
	}
}

/// Check if the specified map contains the key
pub struct Has;

impl Operator for Has {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		assert!(args.len() == 2);

		let tag = evaluate(&args[0], scope)?;
		let key = evaluate(&args[1], scope)?;

		let tag = scope.get_object_tag(tag)?;
		let map = scope.get_object(tag);

		Ok(map.contains_key(&key).into())
	}
}

/// Delete the given key at the given map
pub struct Remove;

impl Operator for Remove {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		assert!(args.len() == 2);

		let tag = evaluate(&args[0], scope)?;
		let key = evaluate(&args[1], scope)?;

		let tag = scope.get_object_tag(tag)?;
		let map = scope.get_object_mut(tag);

		Ok(map.remove(&key).into())
	}
}

/// How many entries does this map have?
pub struct Size;

impl Operator for Size {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		assert!(args.len() == 1);

		let tag = evaluate(&args[0], scope)?;
		let tag = scope.get_object_tag(tag)?;

		let map = scope.get_object(tag);
		Ok((map.len() as f32).into())
	}
}

/// Clear the specified map
pub struct Clear;

impl Operator for Clear {
	fn evaluate(&self, args: &[Expression], scope: &mut Scope) -> EggResult<Value> {
		assert!(args.len() == 1);

		let tag = evaluate(&args[0], scope)?;
		let tag = scope.get_object_tag(tag)?;

		let map = scope.get_object_mut(tag);
		Ok({
			map.clear();
			().into()
		})
	}
}
