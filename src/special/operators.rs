use super::SpecialForm;
use crate::{
    evaluator::evaluate,
    expression::{Expression, Value},
};
use std::{collections::HashMap, sync::Mutex};

// Returns the value's type
pub struct TypeOf;

impl<'a> super::SpecialForm<'a> for TypeOf {
    fn evaluate(
        &self,
        args: &[Expression],
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

// Basic add operation
pub struct Add;

impl<'a> super::SpecialForm<'a> for Add {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &Mutex<HashMap<String, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        args.iter()
            .map(|arg| evaluate(arg, scope, special_forms))
            .reduce(|a, b| match (a, b) {
                (Value::Isize(a), Value::Isize(b)) => Value::Isize(a + b),
                _ => panic!("please provide numbers as arguments for mathematical operations"),
            })
            .unwrap_or(Value::Isize(0))
    }
}

// Basic multiply operation
pub struct Multiply;

impl<'a> super::SpecialForm<'a> for Multiply {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &Mutex<HashMap<String, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        args.iter()
            .map(|arg| evaluate(arg, scope, special_forms))
            .reduce(|a, b| match (a, b) {
                (Value::Isize(a), Value::Isize(b)) => Value::Isize(a * b),
                _ => panic!("please provide numbers as arguments for mathematical operations"),
            })
            .unwrap_or(Value::Isize(1))
    }
}

// Basic minus operation
pub struct Subtract;

impl<'a> super::SpecialForm<'a> for Subtract {
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
            (Value::Isize(a), Value::Isize(b)) => Value::Isize(a - b),
            _ => panic!("please provide numbers as arguments for mathematical operations"),
        }
    }
}

// Basic divide operation
pub struct Divide;

impl<'a> super::SpecialForm<'a> for Divide {
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
            (Value::Isize(a), Value::Isize(b)) => Value::Isize(a / b),
            _ => panic!("please provide numbers as arguments for mathematical operations"),
        }
    }
}

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

        if val1 == val2 {
            Value::Isize(1)
        } else {
            Value::Isize(0)
        }
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

        if val1 != val2 {
            Value::Isize(1)
        } else {
            Value::Isize(0)
        }
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
            (Value::Isize(a), Value::Isize(b)) => Value::Isize(if a > b { 1 } else { 0 }),
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
            (Value::Isize(a), Value::Isize(b)) => Value::Isize(if a < b { 1 } else { 0 }),
            _ => panic!("please provide numbers as arguments for mathematical operations"),
        }
    }
}
