use crate::expression::Value;
use alloc::{collections::BTreeMap, string::String};

/// Build a new scope containing values required by the runtime.
pub fn default() -> BTreeMap<String, Value> {
	let mut map = BTreeMap::new();

	map.insert("true".into(), true.into());
	map.insert("false".into(), false.into());

	// Globals identifying type
	map.insert("NUMBER".into(), Value::String("__NUMBER".into()));
	map.insert("STRING".into(), Value::String("__STRING".into()));
	map.insert("NIL".into(), Value::String("__NIL".into()));
	map.insert("BOOLEAN".into(), Value::String("__BOOLEAN".into()));

	map
}

// Create an empty scope
pub fn empty() -> BTreeMap<String, Value> {
	BTreeMap::new()
}
