use super::SpecialForm;
use crate::{
    evaluator::evaluate,
    expression::{self, Value},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Defines a new variable
pub struct TableNew;

impl<'a> SpecialForm<'a> for TableNew {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &RefCell<HashMap<Rc<str>, Value>>,
        special_forms: &HashMap<&'a str, Box<dyn SpecialForm<'a> + 'a>>,
    ) -> Value {
        assert_eq!(args.len(), 1);
        let name = &args[0];

        match name {
            expression::Expression::Word { name } => {
                let value = evaluate(&args[1], scope, special_forms);
                let mut map = scope.borrow_mut();

                if map.contains_key(name) {
                    panic!("Attempting to re-declare a variable: {name}")
                } else {
                    // THIS IS BASICALLY A CLONE
                    map.insert(name.clone(), value.clone());
                }

                value
            }
            expression::Expression::Value { value } => match value {
                expression::Value::String(name) => {
                    let value = evaluate(&args[1], scope, special_forms);
                    let mut map = scope.borrow_mut();

                    if map.contains_key(name.as_ref()) {
                        panic!("Attempting to re-declare a variable: {name}")
                    } else {
                        map.insert(name.clone(), value.clone());
                    }

                    value
                }
                expression::Value::Number(_) => {
                    panic!("Numbers cannot be used as variable names as the would conflict with normal numbers")
                }
                expression::Value::Table(_) => {
                    panic!("Tables cannot be used as variable names")
                }
            },
            _ => {
                panic!("Applications cannot be used as variable names");
            }
        }
    }
}
