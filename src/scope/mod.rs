pub(crate) mod functions;
pub(crate) mod object;

pub use functions::FunctionDefinition;

use crate::{
	error::{EggError, EggResult},
	expression::Value,
};
use alloc::{collections::BTreeMap, format};
use arcstr::ArcStr;
use core::f32::consts;

/// A [`Scope`] is responsible for keeping track of script state.
///
/// This includes storing variables, which are plain [`Values`](Value).
/// Function and Objects values are simple indexes|references to [`FunctionDefinitions`](functions::FunctionDefinition) and [`BTreeMaps`](BTreeMap), stored in `ScopeExtras`.
///
/// `ScopeExtras` is only attached to the Global Scope, meaning Functions and Objects are always global, even if defined in a script function.
///
/// The [`default`](Default) scope comes with several constants built-in:
/// ```json
/// {
///      "Boolean": "__TYPE__BOOLEAN",
///      "Function": "__TYPE__FUNCTION",
///      "Nil": "__CONSTANT__NIL",
///      "Number": "__TYPE__NUMBER",
///      "Object": "__TYPE__OBJECT",
///      "String": "__TYPE__STRING",
///      "PI": 3.1415927,
///      "E": 2.7182817,
///      "TAU": 6.2831855,
///      "false": false,
///      "true": true
///  }
/// ```
#[derive(Debug)]
#[allow(private_interfaces)]
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
		source.insert("Number".into(), Value::String(arcstr::literal!("__TYPE__NUMBER")));
		source.insert("String".into(), Value::String(arcstr::literal!("__TYPE__STRING")));
		source.insert("Nil".into(), Value::String(arcstr::literal!("__CONSTANT__NIL")));
		source.insert("Boolean".into(), Value::String(arcstr::literal!("__TYPE__BOOLEAN")));
		source.insert("Function".into(), Value::String(arcstr::literal!("__TYPE__FUNCTION")));
		source.insert("Object".into(), Value::String(arcstr::literal!("__TYPE__OBJECT")));

		// Float constants
		source.insert("PI".into(), consts::PI.into());
		source.insert("TAU".into(), consts::TAU.into());
		source.insert("E".into(), consts::E.into());

		Scope::Global { source, extras: Default::default() }
	}
}

impl Scope {
	/// Check if a variable exists anywhere in the scope chain.
	pub fn exists(&self, key: &str) -> bool {
		match self {
			Scope::Global { source, .. } => source.contains_key(key),
			Scope::Local { overlay, source: parent, .. } => overlay.contains_key(key) || unsafe { parent.as_ref().map(|p| p.exists(key)).unwrap_or(false) },
		}
	}

	/// Check if a variable exists in the current scope. Has similar behaviour to [`exists`](Scope::exists) for the Global Scope.
	/// Used to check if a variable is defined in the current scope, and not in the parent scope.
	pub fn exists_locally(&self, key: &str) -> bool {
		match self {
			Scope::Global { source, .. } => source.contains_key(key),
			Scope::Local { overlay, .. } => overlay.contains_key(key),
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
	pub fn insert(&mut self, key: ArcStr, value: Value) -> EggResult<()> {
		let exists_locally = self.exists_locally(&key);

		match self {
			Scope::Global { source, .. } => {
				if exists_locally {
					return Err(EggError::OperatorComplaint(format!("Variable {} already defined in Global Scope", key)));
				}
				source.insert(key, value);
			}
			Scope::Local { overlay, .. } => {
				if exists_locally {
					return Err(EggError::OperatorComplaint(format!("Variable {} already defined in Global Scope", key)));
				}
				overlay.insert(key, value);
			}
		};

		Ok(())
	}

	/// Updates the value of a variable if it is in the present scope, otherwise updates it in the parent scope.
	pub fn update(&mut self, key: ArcStr, value: Value) {
		let was_local = matches!(self, Scope::Local { overlay, .. } if overlay.contains_key(&key));
		self.delete(&key);

		match self {
			Scope::Global { source, .. } => {
				source.insert(key, value);
			}
			Scope::Local { overlay, source } => {
				if was_local {
					overlay.insert(key, value);
				} else {
					unsafe { source.as_mut().map(|s| s.insert(key, value)) };
				}
			}
		};
	}

	/// Delete a variable if it is the present scope, otherwise delete it from the parent scope.
	pub fn delete(&mut self, key: &str) -> Option<Value> {
		if let Some(Value::Function(index)) = self.get(key) {
			self.delete_function(*index);
		}

		if let Some(Value::Object(tag)) = self.get(key) {
			self.delete_object(*tag);
		}

		match self {
			Scope::Global { source, .. } => source.remove(key),
			Scope::Local { overlay, source: parent, .. } => unsafe { overlay.remove(key).or_else(|| parent.as_mut().and_then(|p| p.delete(key))) },
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

#[derive(Debug, Default)]
pub(crate) struct ScopeExtras {
	maps: BTreeMap<usize, BTreeMap<Value, Value>>,
	functions: BTreeMap<usize, functions::FunctionDefinition>,
	counter: usize,
	_unsend: core::marker::PhantomData<*mut ()>,
}
