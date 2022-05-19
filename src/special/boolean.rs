use super::SpecialForm;
use crate::{
    evaluator::evaluate,
    expression::{Expression, Value},
};
use std::{collections::HashMap, sync::Mutex};

// AND
pub struct AND;

impl<'a> super::SpecialForm<'a> for AND {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &Mutex<HashMap<String, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, special_forms);
        let val2 = evaluate(&args[1], scope, special_forms);

        match (val1, val2) {
            (Value::Isize(a), Value::Isize(b)) if a != 0 && b != 0 => Value::Isize(1),
            (Value::Isize(_), Value::Isize(_b)) => Value::Isize(0),
            _ => panic!("please provide numbers as arguments for boolean operations"),
        }
    }
}

// AND
pub struct OR;

impl<'a> super::SpecialForm<'a> for OR {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &Mutex<HashMap<String, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, special_forms);
        let val2 = evaluate(&args[1], scope, special_forms);

        match (val1, val2) {
            (Value::Isize(a), Value::Isize(b)) if a != 0 || b != 0 => Value::Isize(1),
            (Value::Isize(_), Value::Isize(_)) => Value::Isize(0),
            _ => panic!("please provide numbers as arguments for boolean operations"),
        }
    }
}

// AND
pub struct NOT;

impl<'a> super::SpecialForm<'a> for NOT {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &Mutex<HashMap<String, Value>>,
        special_forms: &mut HashMap<&'a str, Box<(dyn SpecialForm<'a> + 'a)>>,
    ) -> Value {
        assert_eq!(args.len(), 1);
        let value = evaluate(&args[0], scope, special_forms);

        match value {
            Value::Isize(a) if a != 0 => Value::Isize(0),
            Value::Isize(_) => Value::Isize(1),
            _ => panic!("please provide numbers as arguments for boolean operations"),
        }
    }
}
