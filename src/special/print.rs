use std::{collections::HashMap, sync::Mutex};

use super::SpecialForm;
use crate::{
    evaluator::evaluate,
    expression::{self, Value},
};

// Prints it's data and a newline
pub struct PrintLine;

impl<'a> SpecialForm<'a> for PrintLine {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &'a Mutex<HashMap<&'a str, Value>>,
        special_forms: &HashMap<&'a str, Box<dyn SpecialForm<'a> + 'a>>,
    ) -> expression::Value {
        for arg in args {
            match evaluate(arg, scope, special_forms) {
                expression::Value::Isize(num) => println!("{num}"),
                expression::Value::String(string) => println!("{string}"),
            }
        }

        expression::Value::Isize(args.len() as isize)
    }
}

// Prints it's arguments without a newline
pub struct Print;

impl<'a> SpecialForm<'a> for Print {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &'a Mutex<HashMap<&'a str, Value>>,
        special_forms: &HashMap<&'a str, Box<dyn SpecialForm<'a> + 'a>>,
    ) -> expression::Value {
        for arg in args {
            match evaluate(arg, scope, special_forms) {
                expression::Value::Isize(num) => print!("{num}"),
                expression::Value::String(string) => print!("{string}"),
            }
        }

        expression::Value::Isize(args.len() as isize)
    }
}
