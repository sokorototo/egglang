use super::SpecialForm;
use crate::{
    evaluator::evaluate,
    expression::{Expression, Value},
};
use std::{collections::HashMap, sync::Mutex};

// Checks for equality
pub struct Equals;

impl<'a> super::SpecialForm<'a> for Equals {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &Mutex<HashMap<String, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, special_forms);
        let val2 = evaluate(&args[1], scope, special_forms);

        (val1 == val2).into()
    }
}

// Checks for inequality
pub struct NotEquals;

impl<'a> super::SpecialForm<'a> for NotEquals {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &Mutex<HashMap<String, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, special_forms);
        let val2 = evaluate(&args[1], scope, special_forms);

        (val1 != val2).into()
    }
}

// Greater than
pub struct GreaterThan;

impl<'a> super::SpecialForm<'a> for GreaterThan {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &Mutex<HashMap<String, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, special_forms);
        let val2 = evaluate(&args[1], scope, special_forms);

        match (val1, val2) {
            (Value::Isize(a), Value::Isize(b)) => (a > b).into(),
            _ => panic!("please provide numbers as arguments for mathematical operations"),
        }
    }
}

// Greater than
pub struct LessThan;

impl<'a> super::SpecialForm<'a> for LessThan {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &Mutex<HashMap<String, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, special_forms);
        let val2 = evaluate(&args[1], scope, special_forms);

        match (val1, val2) {
            (Value::Isize(a), Value::Isize(b)) => (a < b).into(),
            _ => panic!("please provide numbers as arguments for mathematical operations"),
        }
    }
}
