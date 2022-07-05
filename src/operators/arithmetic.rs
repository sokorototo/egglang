use super::Operator;
use crate::{
    evaluator::evaluate,
    expression::{Expression, Value},
};
use std::collections::HashMap;

// Basic add operation
pub struct Add;

impl Operator for Add {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        args.iter()
            .map(|arg| evaluate(arg, scope, builtins))
            .reduce(|a, b| match (a, b) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                _ => panic!("please provide numbers as arguments for mathematical operations"),
            })
            .unwrap_or(Value::Number(0))
    }
}

// Basic multiply operation
pub struct Multiply;

impl Operator for Multiply {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        args.iter()
            .map(|arg| evaluate(arg, scope, builtins))
            .reduce(|a, b| match (a, b) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
                _ => panic!("please provide numbers as arguments for mathematical operations"),
            })
            .unwrap_or(Value::Number(1))
    }
}

// Basic minus operation
pub struct Subtract;

impl Operator for Subtract {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, builtins);
        let val2 = evaluate(&args[1], scope, builtins);

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => panic!("please provide numbers as arguments for mathematical operations"),
        }
    }
}

// Basic divide operation
pub struct Divide;

impl Operator for Divide {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, builtins);
        let val2 = evaluate(&args[1], scope, builtins);

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => panic!("please provide numbers as arguments for mathematical operations"),
        }
    }
}
