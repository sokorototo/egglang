use super::SpecialForm;
use crate::{
    evaluator::evaluate,
    expression::{Expression, Value},
};
use std::collections::HashMap;

// Basic add operation
pub struct Add;

impl<'a> super::SpecialForm<'a> for Add {
    fn evaluate(
        &self,
        args: &'a [Expression],
        scope: &mut HashMap<String, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        args.iter()
            .map(|arg| evaluate(arg, scope, special_forms))
            .reduce(|a, b| match (a, b) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                _ => panic!("please provide numbers as arguments for mathematical operations"),
            })
            .unwrap_or(Value::Number(0))
    }
}

// Basic multiply operation
pub struct Multiply;

impl<'a> super::SpecialForm<'a> for Multiply {
    fn evaluate(
        &self,
        args: &'a [Expression],
        scope: &mut HashMap<String, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        args.iter()
            .map(|arg| evaluate(arg, scope, special_forms))
            .reduce(|a, b| match (a, b) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
                _ => panic!("please provide numbers as arguments for mathematical operations"),
            })
            .unwrap_or(Value::Number(1))
    }
}

// Basic minus operation
pub struct Subtract;

impl<'a> super::SpecialForm<'a> for Subtract {
    fn evaluate(
        &self,
        args: &'a [Expression],
        scope: &mut HashMap<String, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, special_forms);
        let val2 = evaluate(&args[1], scope, special_forms);

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => panic!("please provide numbers as arguments for mathematical operations"),
        }
    }
}

// Basic divide operation
pub struct Divide;

impl<'a> super::SpecialForm<'a> for Divide {
    fn evaluate(
        &self,
        args: &'a [Expression],
        scope: &mut HashMap<String, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, special_forms);
        let val2 = evaluate(&args[1], scope, special_forms);

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => panic!("please provide numbers as arguments for mathematical operations"),
        }
    }
}
