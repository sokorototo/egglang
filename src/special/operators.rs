use crate::{evaluator::evaluate, expression};
use std::{collections::HashMap, sync::Mutex};

// Checks for equality
pub struct Equals;

impl<'a> super::SpecialForm<'a> for Equals {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &'a Mutex<HashMap<&'a str, expression::Value>>,
        special_forms: &HashMap<&'a str, Box<dyn super::SpecialForm<'a> + 'a>>,
    ) -> expression::Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, special_forms);
        let val2 = evaluate(&args[1], scope, special_forms);

        if val1 == val2 {
            expression::Value::Isize(1)
        } else {
            expression::Value::Isize(0)
        }
    }
}

// Returns the value's type
pub struct TypeOf;

impl<'a> super::SpecialForm<'a> for TypeOf {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &'a Mutex<HashMap<&'a str, expression::Value>>,
        special_forms: &HashMap<&'a str, Box<dyn super::SpecialForm<'a> + 'a>>,
    ) -> expression::Value {
        assert_eq!(args.len(), 1);

        let value = evaluate(&args[0], scope, special_forms);

		  match value {
            expression::Value::Isize(_) => expression::Value::String("__number".to_string()),
            expression::Value::String(_) => expression::Value::String("__string".to_string()),
        }
    }
}
