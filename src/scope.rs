use crate::expression::Value;
use alloc::{collections::BTreeMap, string::String};

pub enum Scope {
	Global(BTreeMap<String, Value>),
	Local { overlay: BTreeMap<String, Value>, parent: *mut BTreeMap<String, Value> },
}

impl Scope {
	/// Check if a variable exists in the current scope or its parent scopes.
	pub fn exists(&self, key: &str) -> bool {
		match self {
			Scope::Global(map) => map.contains_key(key),
			Scope::Local { overlay, parent } => overlay.contains_key(key) || unsafe { parent.as_ref().map(|p| p.contains_key(key)).unwrap_or(false) },
		}
	}

	/// Fetch for a variable in the current scope and its parent scopes.
	pub fn get(&self, key: &str) -> Option<&Value> {
		match self {
			Scope::Global(map) => map.get(key),
			Scope::Local { overlay, parent } => overlay.get(key).or_else(|| unsafe { parent.as_mut().map(|p| p.get(key)).flatten() }),
		}
	}

	/// Mutable fetch for a variable in the current scope and its parent scopes.
	pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
		match self {
			Scope::Global(map) => map.get_mut(key),
			Scope::Local { overlay, parent } => overlay.get_mut(key).or_else(|| unsafe { parent.as_mut().map(|p| p.get_mut(key)).flatten() }),
		}
	}

	/// Insert a new variable into the current scope.
	pub fn insert(&mut self, key: String, value: Value) {
		match self {
			Scope::Global(map) => {
				map.insert(key, value);
			}
			Scope::Local { overlay, .. } => {
				overlay.insert(key, value);
			}
		}
	}

	/// Delete a variable if it is the present scope, otherwise delete it from the parent scope.
	pub fn remove(&mut self, key: &str) -> Option<Value> {
		match self {
			Scope::Global(map) => map.remove(key),
			Scope::Local { overlay, parent } => unsafe { overlay.remove(key).or_else(|| parent.as_mut().map(|p| p.remove(key)).flatten()) },
		}
	}

	/// Create a new local scope.
	pub fn new_local(&mut self) -> Self {
		let parent = match self {
			Scope::Global(g) => g as _,
			Scope::Local { parent, .. } => *parent,
		};

		Scope::Local { overlay: BTreeMap::new(), parent }
	}
}

/// Build a new scope containing values required by the runtime.
pub fn default() -> Scope {
	let mut map = BTreeMap::new();

	map.insert("true".into(), true.into());
	map.insert("false".into(), false.into());

	// Globals identifying type
	map.insert("NUMBER".into(), Value::String("__NUMBER".into()));
	map.insert("STRING".into(), Value::String("__STRING".into()));
	map.insert("NIL".into(), Value::String("__NIL".into()));
	map.insert("BOOLEAN".into(), Value::String("__BOOLEAN".into()));

	Scope::Global(map)
}
