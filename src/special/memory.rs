use super::SpecialForm;
use crate::{
    evaluator::evaluate,
    expression::{self, Value},
};
use std::{collections::HashMap, sync::Mutex};

/// Defines a new variable
pub struct Define;

impl<'a> SpecialForm<'a> for Define {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &Mutex<HashMap<String, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        assert_eq!(args.len(), 2);
        let name = &args[0];

        match name {
            expression::Expression::Word { name } => {
                let value = evaluate(&args[1], scope, special_forms);
                let mut map = scope.lock().unwrap();

                if map.contains_key(name) {
                    panic!("Attempting to re-declare a variable: {name}")
                } else {
                    map.insert(name.clone(), value.clone());
                }

                value
            }
            expression::Expression::Value { value } => match value {
                expression::Value::String(name) => {
                    let value = evaluate(&args[1], scope, special_forms);
                    let mut map = scope.lock().unwrap();

                    if map.contains_key(name) {
                        panic!("Attempting to re-declare a variable: {name}")
                    } else {
                        map.insert(name.clone(), value.clone());
                    }

                    value
                }
                expression::Value::Isize(_) => {
                    panic!("Numbers cannot be used as variable names for obvious reasons")
                }
            },
            _ => {
                panic!("Applications cannot be used as variable names for obvious reasons");
            }
        }
    }
}

/// Mutates an existing variable
pub struct Mutate;

impl<'a> SpecialForm<'a> for Mutate {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &Mutex<HashMap<String, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        assert_eq!(args.len(), 2);
        let name = &args[0];

        match name {
            expression::Expression::Word { name } => {
                let value = evaluate(&args[1], scope, special_forms);
                scope
                    .lock()
                    .unwrap()
                    .get_mut(name)
                    .map(|val| *val = value.clone());

                value
            }
            expression::Expression::Value { value } => match value {
                expression::Value::String(name) => {
                    let value = evaluate(&args[1], scope, special_forms);
                    scope
                        .lock()
                        .unwrap()
                        .get_mut(name)
                        .map(|val| *val = value.clone());

                    value
                }
                expression::Value::Isize(_) => {
                    panic!("Numbers cannot be used as variable names for obvious reasons")
                }
            },
            _ => {
                panic!("Applications cannot be used as variable names for obvious reasons");
            }
        }
    }
}

/// Deletes an existing variable
pub struct Delete;

impl<'a> SpecialForm<'a> for Delete {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &Mutex<HashMap<String, Value>>,
        _: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        assert_eq!(args.len(), 1);
        let name = &args[0];

        let res = match name {
            expression::Expression::Word { name } => scope.lock().unwrap().remove(name),
            expression::Expression::Value { value } => match value {
                expression::Value::String(name) => scope.lock().unwrap().remove(name),
                expression::Value::Isize(_) => {
                    panic!("Numbers cannot be used as variable names for obvious reasons")
                }
            },
            _ => {
                panic!("Applications cannot be used as variable names for obvious reasons");
            }
        };

        res.unwrap_or(expression::Value::Isize(-1))
    }
}

// Returns the value's type
pub struct TypeOf;

impl<'a> super::SpecialForm<'a> for TypeOf {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &Mutex<HashMap<String, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 1);

        let value = evaluate(&args[0], scope, special_forms);

        match value {
            Value::Isize(_) => Value::String("__number".to_string()),
            Value::String(_) => Value::String("__string".to_string()),
        }
    }
}
