use super::SpecialForm;
use crate::{
    evaluator::evaluate,
    expression::{self, Value},
};
use std::{collections::HashMap, sync::Mutex};

// Task evaluator
pub struct Do;

impl<'a> SpecialForm<'a> for Do {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &'a Mutex<HashMap<&'a str, Value>>,
        special_forms: &HashMap<&'a str, Box<dyn SpecialForm<'a> + 'a>>,
    ) -> expression::Value {
        let mut result = expression::Value::Isize(-1);

        args.into_iter().for_each(|arg| {
            result = evaluate(arg, scope, special_forms);
        });

        result
    }
}

// Simple if statement
pub struct If;

impl<'a> SpecialForm<'a> for If {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &'a Mutex<HashMap<&'a str, Value>>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        // Assert correct length of arguments
        if args.len() != 3 {
            panic!(
                "If has wrong number of arguments: {}, expected 3",
                args.len()
            );
        }

        // Evaluate
        let res = evaluate(&args[0], scope, special_forms);
        let value = match res {
            expression::Value::Isize(num) => num == 0,
            expression::Value::String(str) => {
                panic!("Expected `isize` found String: {str}")
            }
        };

        if value {
            evaluate(&args[1], scope, special_forms)
        } else {
            evaluate(&args[2], scope, special_forms)
        }
    }
}

// Simple while loop
pub struct While;

impl<'a> SpecialForm<'a> for While {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &'a Mutex<HashMap<&'a str, Value>>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        // Assert correct length of arguments
        if args.len() != 2 {
            panic!(
                "While has wrong number of arguments: {}, expected 2",
                args.len()
            );
        }

        // Loop
        let mut iterations = 0usize;
        let mut loop_value = expression::Value::Isize(-1);

        loop {
            if iterations >= usize::MAX {
                panic!("Max loop iterations met");
            }

            let res = evaluate(&args[0], scope, special_forms);
            let value = match res {
                expression::Value::Isize(num) => num == 0,
                expression::Value::String(str) => {
                    panic!("Expected `isize` found String: {str}")
                }
            };

            if !value {
                break loop_value;
            }

            // Evaluate expression
            loop_value = evaluate(&args[1], scope, special_forms);

            iterations += 1;
        }
    }
}

// Simpler loop construct
pub struct Repeat;

impl<'a> SpecialForm<'a> for Repeat {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &'a Mutex<HashMap<&'a str, Value>>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        // Assert correct length of arguments
        if args.len() != 2 {
            panic!(
                "Repeat has wrong number of arguments: {}, expected 2",
                args.len()
            );
        }

        // Loop
        let mut iterations = 0;
        let mut loop_value = expression::Value::Isize(-1);

        let max_iter = match evaluate(&args[0], scope, special_forms) {
            Value::Isize(num) => num,
            Value::String(_) => panic!("Repeat expects a number as it's argument"),
        };

        loop {
            // Repeat X times
            if iterations >= max_iter {
                break loop_value;
            }

            if iterations >= isize::MAX {
                panic!("Max loop iterations met");
            }

            // Evaluate expression
            loop_value = evaluate(&args[1], scope, special_forms);

            iterations += 1;
        }
    }
}
