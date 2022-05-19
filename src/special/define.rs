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
                scope.lock().unwrap().insert(name.clone(), value.clone());

                value
            }
            expression::Expression::Value { value } => match value {
                expression::Value::String(name) => {
                    let value = evaluate(&args[1], scope, special_forms);
                    scope.lock().unwrap().insert(name.clone(), value.clone());

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
