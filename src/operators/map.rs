#![allow(clippy::upper_case_acronyms)]

use super::Operator;
use crate::{
    evaluator::evaluate,
    expression::{Expression, Value},
};
use std::collections::HashMap;

static mut RESOLVER: Option<HashMap<Value, HashMap<Value, Value>>> = None;

fn get_resolver() -> &'static mut HashMap<Value, HashMap<Value, Value>> {
    unsafe { RESOLVER.get_or_insert(Default::default()) }
}

/// Creates a new Map and binds it to the specified Value.
pub struct NewMap;

impl Operator for NewMap {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // map_ref
        assert!(args.len() == 1);

        let map_ref = evaluate(&args[0], scope, builtins);

        // Get reference to global map resolver
        get_resolver().insert(map_ref.clone(), HashMap::new());

        map_ref
    }
}

/// Checks if the specified Map exists
pub struct ExistsMap;

impl Operator for ExistsMap {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // map_ref
        assert!(args.len() == 1);

        let map_ref = evaluate(&args[0], scope, builtins);

        // Check if map exists
        get_resolver().contains_key(&map_ref).into()
    }
}

/// Delete the map at the given map_ref
/// Returns true if the map was deleted, false otherwise
pub struct DeleteMap;

impl Operator for DeleteMap {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // map_ref
        assert!(args.len() == 1);

        let map_ref = evaluate(&args[0], scope, builtins);

        // Get reference to global map resolver
        get_resolver().remove(&map_ref).is_some().into()
    }
}

/// Insert a new value into the specified map
/// API maps to Rust's [`HashMap`]
pub struct Insert;

impl Operator for Insert {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // map_ref, key, value
        assert!(args.len() == 3);

        let map_ref = evaluate(&args[0], scope, builtins);
        let key = evaluate(&args[1], scope, builtins);
        let value = evaluate(&args[2], scope, builtins);

        let res = match get_resolver().get_mut(&map_ref) {
            Some(map) => map.insert(key, value),
            None => panic!("No map found with the identifier: {map_ref:?}"),
        };

        res.unwrap_or(Value::Nil)
    }
}

/// Print a Map's value to the console
/// API maps to Rust's [`HashMap`]
pub struct PrintMap;

impl Operator for PrintMap {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // map_ref
        assert!(args.len() == 1);

        let map_ref = evaluate(&args[0], scope, builtins);

        match get_resolver().get(&map_ref) {
            Some(map) => println!("{:#?}", map),
            None => panic!("No map found with the identifier: {map_ref:?}"),
        };

        map_ref
    }
}

/// Fetch a [Value] the specified map
/// API maps to Rust's [`HashMap`]
pub struct Get;

impl Operator for Get {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // map_ref, key
        assert!(args.len() == 2);

        let map_ref = evaluate(&args[0], scope, builtins);
        let key = evaluate(&args[1], scope, builtins);

        let res = match get_resolver().get(&map_ref) {
            Some(map) => map.get(&key),
            None => panic!("No map found with the identifier: {map_ref:?}"),
        };

        res.unwrap_or(&Value::Nil).clone()
    }
}

/// Check if the specified map contains the value
/// API maps to Rust's [`HashMap`]
pub struct Has;

impl Operator for Has {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // map_ref, key
        assert!(args.len() == 2);

        let map_ref = evaluate(&args[0], scope, builtins);
        let key = evaluate(&args[1], scope, builtins);

        let res = match get_resolver().get(&map_ref) {
            Some(map) => map.contains_key(&key),
            None => panic!("No map found with the identifier: {map_ref:?}"),
        };

        res.into()
    }
}

/// Delete the given key at the given map
/// API maps to Rust's [`HashMap`]
pub struct Remove;

impl Operator for Remove {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // map_ref, key
        assert!(args.len() == 2);

        let map_ref = evaluate(&args[0], scope, builtins);
        let key = evaluate(&args[1], scope, builtins);

        let res = match get_resolver().get_mut(&map_ref) {
            Some(map) => map.remove(&key),
            None => panic!("No map found with the identifier: {map_ref:?}"),
        };

        res.unwrap_or(Value::Nil)
    }
}

/// How many entries does this map have?
/// API maps to Rust's [`HashMap`]
pub struct Size;

impl Operator for Size {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // map_ref, key
        assert!(args.len() == 1);

        let map_ref = evaluate(&args[0], scope, builtins);

        let res = match get_resolver().get(&map_ref) {
            Some(map) => map.len(),
            None => panic!("No map found with the identifier: {map_ref:?}"),
        };

        Value::Number(res.try_into().unwrap())
    }
}
