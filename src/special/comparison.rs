use super::SpecialForm;
use crate::{
    evaluator::evaluate,
    expression::{Expression, Value},
};
use std::{collections::HashMap, rc::Rc, cell::RefCell};

// Checks for equality
pub struct Equals;

impl<'a> super::SpecialForm<'a> for Equals {
    fn evaluate(
        &self,
        args: &'a [Expression],
        scope: &RefCell<HashMap<Rc<str>, Value>>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
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
        args: &'a [Expression],
        scope: &RefCell<HashMap<Rc<str>, Value>>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
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
        args: &'a [Expression],
        scope: &RefCell<HashMap<Rc<str>, Value>>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, special_forms);
        let val2 = evaluate(&args[1], scope, special_forms);

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => (a > b).into(),
            _ => panic!("please provide numbers as arguments for mathematical operations"),
        }
    }
}

// Greater than
pub struct LessThan;

impl<'a> super::SpecialForm<'a> for LessThan {
    fn evaluate(
        &self,
        args: &'a [Expression],
        scope: &RefCell<HashMap<Rc<str>, Value>>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, special_forms);
        let val2 = evaluate(&args[1], scope, special_forms);

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => (a < b).into(),
            _ => panic!("please provide numbers as arguments for mathematical operations"),
        }
    }
}
