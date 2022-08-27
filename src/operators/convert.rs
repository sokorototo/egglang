use std::collections::HashMap;

use super::Operator;
use crate::{
    evaluator::evaluate,
    expression::{self, Value},
};

pub struct ToString;

impl Operator for ToString {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> expression::Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 1);

        // Evaluate
        let res = evaluate(&args[0], scope, builtins);
        let value = match res {
            expression::Value::Number(number) => number.to_string(),
            x if matches!(x, expression::Value::String(..)) => return x,
            _ => panic!("Cannot convert Nil to a String"),
        };

        expression::Value::String(value.into())
    }
}

// Define a special form that converts strings to numbers
pub struct ToNumber;

impl Operator for ToNumber {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> expression::Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 1);

        // Evaluate
        let res = evaluate(&args[0], scope, builtins);
        let value = match res {
            expression::Value::String(string) => string.parse::<isize>().unwrap(),
            x if matches!(x, expression::Value::Number(..)) => return x,
            _ => panic!("Cannot convert Nil to a number"),
        };

        expression::Value::Number(value)
    }
}
