use crate::{
	errors::{EggError, EggResult},
	expression::Value,
};

use alloc::collections::BTreeMap;
use arcstr::ArcStr;

/// Extra data stored on the scope, like function definitions and dictionaries
#[derive(Debug, Default)]
pub struct ScopeExtras {
	maps: BTreeMap<ArcStr, BTreeMap<Value, Value>>,
}

fn validate_map_tag(value: Value) -> EggResult<ArcStr> {
	match value {
		Value::String(s) => Ok(s),
		i => Err(EggError::InvalidMapTag(i, "Map tag must be a string".into())),
	}
}

impl ScopeExtras {
	pub fn contains_map(&self, tag: Value) -> EggResult<bool> {
		let tag = validate_map_tag(tag)?;
		Ok(self.maps.contains_key(&tag))
	}

	pub fn new_map(&mut self, tag: Value) -> EggResult<ArcStr> {
		let tag = validate_map_tag(tag)?;

		if self.maps.contains_key(&tag) {
			Err(EggError::InvalidMapTag(tag.into(), "Map tag already exists".into()))
		} else {
			self.maps.insert(tag.clone(), BTreeMap::new());
			Ok(tag)
		}
	}

	#[cfg(feature = "std")]
	pub fn print_map(&self, tag: Value) -> EggResult<()> {
		let tag = validate_map_tag(tag)?;
		let map = self.maps.get(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		println!("{:#?}", map);
		Ok(())
	}

	pub fn delete_map(&mut self, tag: Value) -> EggResult<bool> {
		let tag = validate_map_tag(tag)?;
		Ok(self.maps.remove(&tag).is_some())
	}

	pub fn get(&self, map_tag: Value, key: Value) -> EggResult<Value> {
		let tag = validate_map_tag(map_tag)?;
		let map = self.maps.get(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		Ok(map.get(&key).cloned().unwrap_or(Value::Nil))
	}

	pub fn insert(&mut self, map_tag: Value, key: Value, value: Value) -> EggResult<Option<Value>> {
		let tag = validate_map_tag(map_tag)?;
		let map = self.maps.get_mut(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		Ok(map.insert(key, value))
	}

	pub fn has(&self, map_tag: Value, key: Value) -> EggResult<bool> {
		let tag = validate_map_tag(map_tag)?;
		let map = self.maps.get(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		Ok(map.contains_key(&key))
	}

	pub fn remove(&mut self, map_tag: Value, key: Value) -> EggResult<Option<Value>> {
		let tag = validate_map_tag(map_tag)?;
		let map = self.maps.get_mut(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		Ok(map.remove(&key))
	}

	pub fn clear(&mut self, map_tag: Value) -> EggResult<()> {
		let tag = validate_map_tag(map_tag)?;
		let map = self.maps.get_mut(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		map.clear();
		Ok(())
	}

	pub fn size(&self, map_tag: Value) -> EggResult<usize> {
		let tag = validate_map_tag(map_tag)?;
		let map = self.maps.get(&tag).ok_or_else(|| EggError::MapNotFound(tag))?;
		Ok(map.len())
	}
}
