use super::SpecialForm;
use crate::{
    evaluator::evaluate,
    expression::{self, Value},
};
use std::collections::HashMap;

/// Defines a new variable
pub struct Define;

impl<'a> SpecialForm<'a> for Define {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &mut HashMap<String, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        assert_eq!(args.len(), 2);
        let name = &args[0];

        match name {
            expression::Expression::Word { name } => {
                let value = evaluate(&args[1], scope, special_forms);

                if scope.contains_key(name.as_ref()) {
                    panic!("Attempting to re-declare a variable: {name}")
                } else {
                    // THIS IS BASICALLY A CLONE
                    scope.insert(name.to_string(), value.clone());
                }

                value
            }
            expression::Expression::Value { value } => match value {
                expression::Value::String(name) => {
                    let value = evaluate(&args[1], scope, special_forms);

                    if scope.contains_key(name.as_ref()) {
                        panic!("Attempting to re-declare a variable: {name}")
                    } else {
                        scope.insert(name.to_string(), value.clone());
                    }

                    value
                }
                _ => panic!("Numbers cannot be used as variable names as the would conflict with normal numbers")
            },
            _ => {
                panic!("Applications cannot be used as variable names");
            }
        }
    }
}

/// Mutates an existing variable
pub struct Mutate;

impl<'a> SpecialForm<'a> for Mutate {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &mut HashMap<String, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        assert_eq!(args.len(), 2);
        let variable_name = &args[0];
        let old_value;

        match variable_name {
            expression::Expression::Word { name: word } => {
                let value = evaluate(&args[1], scope, special_forms);
                old_value = evaluate(variable_name, scope, special_forms);

                if let Some(val) = scope.get_mut(word.as_ref()) {
                    *val = value;
                };
            }
            expression::Expression::Value { value: string } => match string {
                expression::Value::String(name) => {
                    let value = evaluate(&args[1], scope, special_forms);

                    if let Some(val) = scope.get_mut(name.as_ref()) {
                        *val = value.clone()
                    };

                    old_value = value;
                }
                _ => panic!("Please provide a word or a string as a variable name"),
            },
            _ => {
                panic!("Applications cannot be used as variable names for obvious reasons");
            }
        }

        old_value
    }
}

/// Deletes an existing variable
pub struct Delete;

impl<'a> SpecialForm<'a> for Delete {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        _: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        assert_eq!(args.len(), 1);
        let name = &args[0];

        let res = match name {
            expression::Expression::Word { name } => scope.remove(name.as_ref()),
            expression::Expression::Value { value } => match value {
                expression::Value::String(name) => scope.remove(name.as_ref()),
                val => panic!("Invalid value given to delete: {val:?}"),
            },
            _ => {
                panic!("Applications cannot be used as variable names for obvious reasons");
            }
        };

        res.unwrap_or(expression::Value::Number(-1))
    }
}

/// Checks if a variable exists
pub struct Exists;

impl<'a> SpecialForm<'a> for Exists {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        _: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        assert_eq!(args.len(), 1);
        let name = &args[0];

        let res = match name {
            expression::Expression::Word { name } => scope.contains_key(name.as_ref()),
            expression::Expression::Value { value } => match value {
                expression::Value::String(name) => scope.contains_key(name.as_ref()),
                val => panic!("Invalid value passed to exists(--): {val:?}"),
            },
            _ => {
                panic!("Applications cannot be used as variable names for obvious reasons");
            }
        };

        res.into()
    }
}

// Returns the value's type
pub struct TypeOf;

impl<'a> super::SpecialForm<'a> for TypeOf {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &mut HashMap<String, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 1);

        let value = evaluate(&args[0], scope, special_forms);

        match value {
            Value::Number(_) => Value::String("__number".into()),
            Value::String(_) => Value::String("__string".into()),
        }
    }
}
