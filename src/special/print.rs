use std::{collections::HashMap, sync::Mutex, rc::Rc};

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
        scope: &Mutex<HashMap<Rc<str>, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        for arg in args {
            match evaluate(arg, scope, special_forms) {
                expression::Value::Number(num) => println!("{num}"),
                expression::Value::String(string) => println!("{string}"),
            }
        }

        expression::Value::Number(args.len() as isize)
    }
}

// Prints it's arguments without a newline
pub struct Print;

impl<'a> SpecialForm<'a> for Print {
    fn evaluate(
        &self,
        args: &'a [expression::Expression],
        scope: &Mutex<HashMap<Rc<str>, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> expression::Value {
        for arg in args {
            match evaluate(arg, scope, special_forms) {
                expression::Value::Number(num) => print!("{num}"),
                expression::Value::String(string) => print!("{string}"),
            }
        }

        expression::Value::Number(args.len() as isize)
    }
}
