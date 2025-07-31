use alloc::{collections::BTreeMap, format};
use arcstr::ArcStr;

use crate::{
	error::{EggError, EggResult},
	expression::Value,
};

pub(crate) mod functions;
pub(crate) mod object;

/// A [`Scope`] is responsible for keeping track of script state.
///
/// This includes storing variables, which are plain [`Values`](Value).
/// Function and Objects values are simple indexes|references to [`FunctionDefinitions`](functions::FunctionDefinition) and [`BTreeMaps`](BTreeMap), stored in [`Extras`].
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
///      "false": false,
///      "true": true
///  }
/// ```
#[derive(Debug)]
#[allow(private_interfaces)]
pub enum Scope {
	Global { source: BTreeMap<ArcStr, Value>, extras: Extras },
	Local { overlay: BTreeMap<ArcStr, Value>, source: *mut Scope },
}

impl Default for Scope {
	fn default() -> Scope {
		let mut source = BTreeMap::new();

		source.insert(arcstr::literal!("true"), true.into());
		source.insert(arcstr::literal!("false"), false.into());

		// type names
		source.insert(arcstr::literal!("Number"), Value::String(arcstr::literal!("__TYPE__NUMBER")));
		source.insert(arcstr::literal!("String"), Value::String(arcstr::literal!("__TYPE__STRING")));
		source.insert(arcstr::literal!("Nil"), Value::String(arcstr::literal!("__CONSTANT__NIL")));
		source.insert(arcstr::literal!("Boolean"), Value::String(arcstr::literal!("__TYPE__BOOLEAN")));
		source.insert(arcstr::literal!("Function"), Value::String(arcstr::literal!("__TYPE__FUNCTION")));
		source.insert(arcstr::literal!("Object"), Value::String(arcstr::literal!("__TYPE__OBJECT")));

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

	/// Used to check if a variable is defined in the current scope, and specifically not in the parent scope.
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
		if self.exists_locally(&key) {
			return Err(EggError::OperatorComplaint(format!("Variable {} already defined", key)));
		}

		match self {
			Scope::Global { source, .. } => source.insert(key, value),
			Scope::Local { overlay, .. } => overlay.insert(key, value),
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
	pub(crate) fn local(&mut self, overlay: BTreeMap<ArcStr, Value>) -> Scope {
		Scope::Local { overlay, source: self as _ }
	}

	/// Get extra metadata attached to the scope.
	pub(crate) fn extras(&self) -> &Extras {
		match self {
			Scope::Global { extras, .. } => extras,
			Scope::Local { source, .. } => unsafe { source.as_ref().map(|s| s.extras()).unwrap_unchecked() },
		}
	}

	/// Get mutable extra metadata attached to the scope.
	pub(crate) fn extras_mut(&mut self) -> &mut Extras {
		match self {
			Scope::Global { extras, .. } => extras,
			Scope::Local { source, .. } => unsafe { source.as_mut().map(|s| s.extras_mut()).unwrap_unchecked() },
		}
	}
}

// TODO: Make objects local and reference counted, whilst forbidding self-reference
#[derive(Debug, Default)]
pub(crate) struct Extras {
	maps: BTreeMap<usize, BTreeMap<Value, Value>>,
	functions: BTreeMap<usize, functions::FunctionDefinition>,
	counter: usize,
	_unsend: core::marker::PhantomData<*mut ()>,
}
