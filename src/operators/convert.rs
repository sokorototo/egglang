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
            _ => panic!("to_string expects a number as it's parameter"),
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
            _ => panic!("to_number expects a string as it's parameter"),
        };

        expression::Value::Number(value)
    }
}
