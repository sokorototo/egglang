use super::Operator;
use crate::{
    evaluator::evaluate,
    expression::{self, Value},
};
use std::collections::HashMap;

// Evaluates all expressions defined within it's operands
pub struct Do;

impl Operator for Do {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> expression::Value {
        let mut result = expression::Value::Nil;

        for arg in args.iter() {
            result = evaluate(arg, scope, builtins);
        }

        result
    }
}

// Simple if statement
pub struct If;

impl Operator for If {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 3);

        // Evaluate
        let res = evaluate(&args[0], scope, builtins);
        let value = match res {
            expression::Value::Number(num) => num != 0,
            _ => panic!("if(--) expects a number as it's parameter"),
        };

        if value {
            evaluate(&args[1], scope, builtins)
        } else {
            evaluate(&args[2], scope, builtins)
        }
    }
}

// Simple while loop
pub struct While;

impl Operator for While {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 2);

        // Loop
        let mut iterations = 0usize;
        let mut loop_result = expression::Value::Nil;

        loop {
            if iterations == usize::MAX {
                panic!("Max loop iterations met");
            }

            let condition = evaluate(&args[0], scope, builtins);
            let continue_condition = match condition {
                expression::Value::Number(num) => num != 0,
                _ => panic!("while(--) expects a number as it's parameter"),
            };

            if !continue_condition {
                break loop_result;
            }

            // Evaluate expression
            loop_result = evaluate(&args[1], scope, builtins);

            iterations += 1;
        }
    }
}

// Simpler loop construct
pub struct Repeat;

impl Operator for Repeat {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 2);

        // Loop
        let mut iterations = 0;
        let mut loop_value = expression::Value::Nil;

        let max_iter = match evaluate(&args[0], scope, builtins) {
            Value::Number(num) => num,
            _ => panic!("repeat(--) expects a number as it's argument"),
        };

        loop {
            // Repeat X times
            if iterations >= max_iter || iterations == isize::MAX {
                break loop_value;
            }

            // Evaluate expression
            loop_value = evaluate(&args[1], scope, builtins);

            iterations += 1;
        }
    }
}

// Sleep for x milliseconds
pub struct Sleep;

impl Operator for Sleep {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        use std::{thread::sleep, time::Duration};

        // Assert correct length of arguments
        assert_eq!(args.len(), 1);

        // Loop
        let sleep_time = evaluate(&args[0], scope, builtins);
        if let Value::Number(value) = sleep_time {
            if value < 0 {
                panic!("Cannot call sleep(--) with a negative time");
            } else {
                let duration = Duration::from_millis(value as u64);
                sleep(duration)
            }
        } else {
            panic!("Please provide a number as the parameter to sleep(--)")
        }

        sleep_time
    }
}

// Sleep for x milliseconds
pub struct Panic;

impl Operator for Panic {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 1);

        // Loop
        let error_message = evaluate(&args[0], scope, builtins);

        match error_message {
            Value::Number(error_code) => {
                panic!("Program has met an unexpected error: ErrorCode: {error_code}")
            }
            Value::String(message) => panic!("{message}"),
            Value::Nil => panic!("Program has gracefully exited"),
        }
    }
}

pub struct Assert;

impl Operator for Assert {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 2);

        // Loop
        let assertion = evaluate(&args[0], scope, builtins);
        match assertion {
            Value::Number(value) => {
                if value == 0 {
                    let error_messsage = evaluate(&args[1], scope, builtins);

                    match error_messsage {
                        Value::Number(i) => panic!("Assertion failed: Error Code given = [{i}]"),
                        Value::String(msg) => panic!("{msg}"),
                        Value::Nil => panic!("Assertion Failed! No residual value provided"),
                    }
                }
            }
            _ => panic!("-assert- takes a boolean (basically an int that equals zero) as it's first argument"),
        };

        Value::Nil
    }
}
