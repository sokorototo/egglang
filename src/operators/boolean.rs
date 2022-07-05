#![allow(clippy::upper_case_acronyms)]

use super::Operator;
use crate::{
    evaluator::evaluate,
    expression::{Expression, Value},
};
use std::collections::HashMap;

// AND
pub struct AND;

impl Operator for AND {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, builtins);
        let val2 = evaluate(&args[1], scope, builtins);

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => (a != 0 && b != 0).into(),
            _ => panic!("please provide numbers as arguments for boolean operations"),
        }
    }
}

// AND
pub struct OR;

impl Operator for OR {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, builtins);
        let val2 = evaluate(&args[1], scope, builtins);

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => (a != 0 || b != 0).into(),
            _ => panic!("please provide numbers as arguments for boolean operations"),
        }
    }
}

// AND
pub struct NOT;

impl Operator for NOT {
    fn evaluate(
        &self,
        args: &[Expression],
        scope: &mut HashMap<String, Value>,
        builtins: &HashMap<&str, Box<dyn Operator>>,
    ) -> Value {
        assert_eq!(args.len(), 1);
        let value = evaluate(&args[0], scope, builtins);

        match value {
            Value::Number(a) => (a == 0).into(),
            _ => panic!("please provide numbers as arguments for boolean operations"),
        }
    }
}
