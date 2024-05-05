mod extras;

use crate::expression::Value;
use alloc::{collections::BTreeMap, string::String};

#[derive(Debug)]
pub enum Scope {
	Global { source: BTreeMap<String, Value>, extras: extras::ScopeExtras },
	Local { overlay: BTreeMap<String, Value>, source: *mut Scope },
}

impl Default for Scope {
	fn default() -> Scope {
		let mut source = BTreeMap::new();

		source.insert("true".into(), true.into());
		source.insert("false".into(), false.into());

		// Globals identifying type
		source.insert("NUMBER".into(), Value::String("__NUMBER".into()));
		source.insert("STRING".into(), Value::String("__STRING".into()));
		source.insert("NIL".into(), Value::String("__NIL".into()));
		source.insert("BOOLEAN".into(), Value::String("__BOOLEAN".into()));

		Scope::Global { source, extras: Default::default() }
	}
}

impl Scope {
	/// Check if a variable exists in the current scope or its parent scopes.
	pub fn exists(&self, key: &str) -> bool {
		match self {
			Scope::Global { source, .. } => source.contains_key(key),
			Scope::Local { overlay, source: parent, .. } => overlay.contains_key(key) || unsafe { parent.as_ref().map(|p| p.exists(key)).unwrap_or(false) },
		}
	}

	/// Fetch for a variable in the current scope and its parent scopes.
	pub fn get(&self, key: &str) -> Option<&Value> {
		match self {
			Scope::Global { source, .. } => source.get(key),
			Scope::Local { overlay, source: parent, .. } => overlay.get(key).or_else(|| unsafe { parent.as_mut().map(|p| p.get(key)).flatten() }),
		}
	}

	/// Mutable fetch for a variable in the current scope and its parent scopes.
	pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
		match self {
			Scope::Global { source, .. } => source.get_mut(key),
			Scope::Local { overlay, source: parent, .. } => overlay.get_mut(key).or_else(|| unsafe { parent.as_mut().map(|p| p.get_mut(key)).flatten() }),
		}
	}

	/// Insert a new variable into the current scope.
	pub fn insert(&mut self, key: String, value: Value) {
		match self {
			Scope::Global { source, .. } => {
				source.insert(key, value);
			}
			Scope::Local { overlay, .. } => {
				overlay.insert(key, value);
			}
		}
	}

	/// Delete a variable if it is the present scope, otherwise delete it from the parent scope.
	pub fn remove(&mut self, key: &str) -> Option<Value> {
		match self {
			Scope::Global { source, .. } => source.remove(key),
			Scope::Local { overlay, source: parent, .. } => unsafe { overlay.remove(key).or_else(|| parent.as_mut().map(|p| p.remove(key)).flatten()) },
		}
	}

	/// Create a new local scope.
	pub fn new_local(&mut self) -> Scope {
		Scope::Local {
			overlay: Default::default(),
			source: self as _,
		}
	}

	/// Get extra metadata attached to the scope.
	pub fn extras(&self) -> &extras::ScopeExtras {
		match self {
			Scope::Global { extras, .. } => extras,
			Scope::Local { source, .. } => unsafe { source.as_ref().map(|s| s.extras()).unwrap_unchecked() },
		}
	}

	/// Get mutable extra metadata attached to the scope.
	pub fn extras_mut(&mut self) -> &mut extras::ScopeExtras {
		match self {
			Scope::Global { extras, .. } => extras,
			Scope::Local { source, .. } => unsafe { source.as_mut().map(|s| s.extras_mut()).unwrap_unchecked() },
		}
	}
}