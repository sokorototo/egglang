use std::collections::HashMap;

use super::Operator;
use crate::{
    evaluator::evaluate,
    expression::{self, Value},
};
// Prints it's data and a newline
pub struct PrintLine;

impl Operator for PrintLine {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> expression::Value {
        for arg in args {
            match evaluate(arg, scope, builtins) {
                expression::Value::Number(num) => println!("{num}"),
                expression::Value::String(string) => println!("{string}"),
                expression::Value::Nil => println!("nil"),
            }
        }

        expression::Value::Number(args.len() as isize)
    }
}

// Prints it's arguments without a newline
pub struct Print;

impl Operator for Print {
    fn evaluate(
        &self,
        args: &[expression::Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> expression::Value {
        for arg in args {
            match evaluate(arg, scope, builtins) {
                expression::Value::Number(num) => print!("{num} "),
                expression::Value::String(string) => print!("{string} "),
                expression::Value::Nil => print!("nil "),
            }
        }

        expression::Value::Number(args.len() as isize)
    }
}
