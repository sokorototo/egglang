pub mod functions;
pub mod object;

use crate::expression::Value;
use alloc::collections::BTreeMap;
use arcstr::ArcStr;
use core::f32::consts;

#[derive(Debug)]
pub enum Scope {
	Global { source: BTreeMap<ArcStr, Value>, extras: ScopeExtras },
	Local { overlay: BTreeMap<ArcStr, Value>, source: *mut Scope },
}

impl Default for Scope {
	fn default() -> Scope {
		let mut source = BTreeMap::new();

		source.insert("true".into(), true.into());
		source.insert("false".into(), false.into());

		// Globals identifying type
		source.insert("Number".into(), Value::String("__TYPE__NUMBER".into()));
		source.insert("String".into(), Value::String("__TYPE__STRING".into()));
		source.insert("Nil".into(), Value::String("__CONSTANT__NIL".into()));
		source.insert("Boolean".into(), Value::String("__TYPE__BOOLEAN".into()));
		source.insert("Function".into(), Value::String("__TYPE__FUNCTION".into()));
		source.insert("Object".into(), Value::String("__TYPE__OBJECT".into()));

		// Float constants
		source.insert("PI".into(), consts::PI.into());
		source.insert("TAU".into(), consts::TAU.into());
		source.insert("E".into(), consts::E.into());

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
			Scope::Local { overlay, source: parent, .. } => overlay.get(key).or_else(|| unsafe { parent.as_mut().and_then(|p| p.get(key)) }),
		}
	}

	/// Mutable fetch for a variable in the current scope and its parent scopes.
	pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
		match self {
			Scope::Global { source, .. } => source.get_mut(key),
			Scope::Local { overlay, source: parent, .. } => overlay.get_mut(key).or_else(|| unsafe { parent.as_mut().and_then(|p| p.get_mut(key)) }),
		}
	}

	/// Insert a new variable into the current scope.
	pub fn insert(&mut self, key: ArcStr, value: Value) {
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
		if let Some(Value::Function(index)) = self.get(key) {
			self.delete_function(*index);
		}

		if let Some(Value::Object(tag)) = self.get(key) {
			// the borrow checker can be annoying
			let tag = tag.clone();
			self.extras_mut().maps.remove(&tag);
		}

		match self {
			Scope::Global { source, .. } => source.remove(key),
			Scope::Local { overlay, source: parent, .. } => unsafe { overlay.remove(key).or_else(|| parent.as_mut().and_then(|p| p.remove(key))) },
		}
	}

	/// Create a new local scope.
	pub(crate) fn overlay(&mut self, overlay: BTreeMap<ArcStr, Value>) -> Scope {
		Scope::Local { overlay, source: self as _ }
	}

	/// Get extra metadata attached to the scope.
	pub(crate) fn extras(&self) -> &ScopeExtras {
		match self {
			Scope::Global { extras, .. } => extras,
			Scope::Local { source, .. } => unsafe { source.as_ref().map(|s| s.extras()).unwrap_unchecked() },
		}
	}

	/// Get mutable extra metadata attached to the scope.
	pub(crate) fn extras_mut(&mut self) -> &mut ScopeExtras {
		match self {
			Scope::Global { extras, .. } => extras,
			Scope::Local { source, .. } => unsafe { source.as_mut().map(|s| s.extras_mut()).unwrap_unchecked() },
		}
	}
}

/// Extra data stored on the scope, like function definitions and dictionaries.
/// Only the Global scope has this, meaning Objects and Functions are accessible globally. Even when defined in a local scope.
/// You will still need access to the Function or Object's reference [`Value`]
#[derive(Debug, Default)]
pub struct ScopeExtras {
	maps: BTreeMap<usize, BTreeMap<Value, Value>>,
	current_object_index: usize,
	functions: BTreeMap<usize, functions::FunctionDefinition>,
	current_function_index: usize,
}
