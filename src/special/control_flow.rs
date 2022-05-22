use super::SpecialForm;
use crate::{
    evaluator::evaluate,
    expression::{self, Value},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Block evaluator
pub struct Do;

impl<'a> SpecialForm<'a> for Do {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &RefCell<HashMap<Rc<str>, Value>>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        let mut result = expression::Value::Number(-1);

        for arg in args.iter() {
            result = evaluate(arg, scope, special_forms);
        }

        result
    }
}

// Simple if statement
pub struct If;

impl<'a> SpecialForm<'a> for If {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &RefCell<HashMap<Rc<str>, Value>>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 3);

        // Evaluate
        let res = evaluate(&args[0], scope, special_forms);
        let value = match res {
            expression::Value::Number(num) => num != 0,
            _ => panic!("if(--) expects a number as it's parameter"),
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
        scope: &RefCell<HashMap<Rc<str>, Value>>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 2);

        // Loop
        let mut iterations = 0usize;
        let mut loop_value = expression::Value::Number(-1);

        loop {
            if iterations == usize::MAX {
                panic!("Max loop iterations met");
            }

            let res = evaluate(&args[0], scope, special_forms);
            let value = match res {
                expression::Value::Number(num) => num == 0,
                _ => panic!("while(--) expects a number as it's parameter"),
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
        scope: &RefCell<HashMap<Rc<str>, Value>>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 2);

        // Loop
        let mut iterations = 0;
        let mut loop_value = expression::Value::Number(-1);

        let max_iter = match evaluate(&args[0], scope, special_forms) {
            Value::Number(num) => num,
            _ => panic!("repeat(--) expects a number as it's argument"),
        };

        loop {
            // Repeat X times
            if iterations >= max_iter || iterations == isize::MAX {
                break loop_value;
            }

            // Evaluate expression
            loop_value = evaluate(&args[1], scope, special_forms);

            iterations += 1;
        }
    }
}

// Sleep for x milliseconds
pub struct Sleep;

impl<'a> SpecialForm<'a> for Sleep {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &RefCell<HashMap<Rc<str>, Value>>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        use std::{thread::sleep, time::Duration};

        // Assert correct length of arguments
        assert_eq!(args.len(), 1);

        // Loop
        let sleep_time = evaluate(&args[0], scope, special_forms);
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

impl<'a> SpecialForm<'a> for Panic {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &RefCell<HashMap<Rc<str>, Value>>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        // Assert correct length of arguments
        assert_eq!(args.len(), 1);

        // Loop
        let error_message = evaluate(&args[0], scope, special_forms);

        match error_message {
            Value::Number(error_code) => {
                panic!("Program has met an unexpected error: ErrorCode: {error_code}")
            }
            Value::String(message) => panic!("{message}"),
        }
    }
}
