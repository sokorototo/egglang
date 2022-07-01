use std::collections::HashMap;

use super::SpecialForm;
use crate::{
    evaluator::evaluate,
    expression::{self, Value},
};

pub struct Concat;

impl<'a> SpecialForm<'a> for Concat {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &mut HashMap<String, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        let mut result = String::with_capacity(args.len() * 64);

        for arg in args {
            match evaluate(arg, scope, special_forms) {
                expression::Value::String(string) => result.push_str(&string),
                _ => panic!("concat expects strings as it's parameters"),
            }
        }

        expression::Value::String(result.into())
    }
}

pub struct Length;

impl<'a> SpecialForm<'a> for Length {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &mut HashMap<String, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 1);

        // Evaluate
        let res = evaluate(&args[0], scope, special_forms);
        let value = match res {
            expression::Value::String(string) => string.len(),
            _ => panic!("length expects a string as it's parameter"),
        };

        expression::Value::Number(value as isize)
    }
}

// Define a special form that extracts a slice from a string and produces a new string given a start and a length
pub struct Slice;

impl<'a> SpecialForm<'a> for Slice {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &mut HashMap<String, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 3);

        // Evaluate
        let res = evaluate(&args[0], scope, special_forms);

        let base = match res {
            expression::Value::String(string) => string,
            _ => panic!("slice expects a string as it's parameter"),
        };

        let start = match evaluate(&args[1], scope, special_forms) {
            expression::Value::Number(num) => num as usize,
            _ => panic!("slice expects a number as it's parameter"),
        };

        let length = match evaluate(&args[2], scope, special_forms) {
            expression::Value::Number(num) => num as usize,
            _ => panic!("slice expects a number as it's parameter"),
        };

        let result = &base[start..start + length];
        expression::Value::String(result.into())
    }
}

// Define two special forms that take a string and convert to uppercase and lowercase respectively
pub struct ToUpper;

impl<'a> SpecialForm<'a> for ToUpper {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &mut HashMap<String, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 1);

        // Evaluate
        let res = evaluate(&args[0], scope, special_forms);
        let value = match res {
            expression::Value::String(string) => string.to_uppercase(),
            _ => panic!("to_upper expects a string as it's parameter"),
        };

        expression::Value::String(value.into())
    }
}

pub struct ToLower;

impl<'a> SpecialForm<'a> for ToLower {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &mut HashMap<String, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 1);

        // Evaluate
        let res = evaluate(&args[0], scope, special_forms);
        let value = match res {
            expression::Value::String(string) => string.to_lowercase(),
            _ => panic!("to_lower expects a string as it's parameter"),
        };

        expression::Value::String(value.into())
    }
}
