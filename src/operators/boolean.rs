#![allow(clippy::upper_case_acronyms)]

use super::Operator;
use crate::{
    errors::{EggError, EggResult},
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
    ) -> EggResult<Value> {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, builtins)?;
        let val2 = evaluate(&args[1], scope, builtins)?;

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => Ok((a != 0 && b != 0).into()),
            _ => Err(EggError::OperatorComplaint(
                "please provide numbers as arguments for boolean operations".to_string(),
            )),
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
    ) -> EggResult<Value> {
        assert_eq!(args.len(), 2);

        let val1 = evaluate(&args[0], scope, builtins)?;
        let val2 = evaluate(&args[1], scope, builtins)?;

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => Ok((a != 0 || b != 0).into()),
            _ => Err(EggError::OperatorComplaint(
                "please provide numbers as arguments for boolean operations".to_string(),
            )),
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
    ) -> EggResult<Value> {
        assert_eq!(args.len(), 1);

        let val = evaluate(&args[0], scope, builtins)?;

        match val {
            Value::Number(a) => Ok((a == 0).into()),
            _ => Err(EggError::OperatorComplaint(
                "please provide numbers as arguments for boolean operations".to_string(),
            )),
        }
    }
}
