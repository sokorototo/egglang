#![allow(clippy::upper_case_acronyms)]

use super::SpecialForm;
use crate::{
    evaluator::evaluate,
    expression::{Expression, Value},
};
use std::{collections::HashMap, rc::Rc};

// AND
pub struct AND;

impl<'a> super::SpecialForm<'a> for AND {
    fn evaluate(
        &self,
        args: &'a [Expression],
        scope: &mut HashMap<Rc<str>, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, special_forms);
        let val2 = evaluate(&args[1], scope, special_forms);

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => (a != 0 && b != 0).into(),
            _ => panic!("please provide numbers as arguments for boolean operations"),
        }
    }
}

// AND
pub struct OR;

impl<'a> super::SpecialForm<'a> for OR {
    fn evaluate(
        &self,
        args: &'a [Expression],
        scope: &mut HashMap<Rc<str>, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, special_forms);
        let val2 = evaluate(&args[1], scope, special_forms);

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => (a != 0 || b != 0).into(),
            _ => panic!("please provide numbers as arguments for boolean operations"),
        }
    }
}

// AND
pub struct NOT;

impl<'a> super::SpecialForm<'a> for NOT {
    fn evaluate(
        &self,
        args: &'a [Expression],
        scope: &mut HashMap<Rc<str>, Value>,
        special_forms: &HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 1);
        let value = evaluate(&args[0], scope, special_forms);

        match value {
            Value::Number(a) => (a != 0).into(),
            _ => panic!("please provide numbers as arguments for boolean operations"),
        }
    }
}
