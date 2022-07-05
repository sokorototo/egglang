use std::collections::HashMap;

use super::Operator;
use crate::{
    evaluator::evaluate,
    expression::{self, Value},
};

pub struct Concat;

impl Operator for Concat {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> expression::Value {
        let mut result = String::with_capacity(args.len() * 64);

        for arg in args {
            match evaluate(arg, scope, builtins) {
                expression::Value::String(string) => result.push_str(&string),
                _ => panic!("concat expects strings as it's parameters"),
            }
        }

        expression::Value::String(result.into())
    }
}

pub struct Length;

impl Operator for Length {
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
            expression::Value::String(string) => string.len(),
            _ => panic!("length expects a string as it's parameter"),
        };

        expression::Value::Number(value as isize)
    }
}

// Define a special form that extracts a slice from a string and produces a new string given a start and a length
pub struct Slice;

impl Operator for Slice {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> expression::Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 3);

        // Evaluate
        let res = evaluate(&args[0], scope, builtins);

        let base = match res {
            expression::Value::String(string) => string,
            _ => panic!("slice expects a string as it's parameter"),
        };

        let mut start = match evaluate(&args[1], scope, builtins) {
            expression::Value::Number(num) => num,
            _ => panic!("slice expects a number as it's parameter"),
        };

        // Negative indeces start from behind
        if start < 0 {
            start = (base.len() as isize) + start;
        }

        let length = match evaluate(&args[2], scope, builtins) {
            expression::Value::Number(num) => num as usize,
            _ => panic!("slice expects a number as it's parameter"),
        };

        let start = start as usize;
        let result = &base[start..start + length];
        expression::Value::String(result.into())
    }
}

// Define two special forms that take a string and convert to uppercase and lowercase respectively
pub struct ToUpper;

impl Operator for ToUpper {
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
            expression::Value::String(string) => string.to_uppercase(),
            _ => panic!("to_upper expects a string as it's parameter"),
        };

        expression::Value::String(value.into())
    }
}

pub struct ToLower;

impl Operator for ToLower {
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
            expression::Value::String(string) => string.to_lowercase(),
            _ => panic!("to_lower expects a string as it's parameter"),
        };

        expression::Value::String(value.into())
    }
}

pub struct Trim;

impl Operator for Trim {
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
        match res {
            expression::Value::String(string) => expression::Value::String(string.trim().into()),
            _ => panic!("trim expects a string as it's parameter"),
        }
    }
}
