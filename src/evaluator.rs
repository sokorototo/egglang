use crate::{
    expression::{Expression, Value},
    operators::Operator,
};
use std::collections::HashMap;

pub static mut EVALUATIONS: u64 = 0;

pub fn evaluate(
    expr: &Expression,
    scope: &mut HashMap<String, Value>,
    builtins: &HashMap<&str, Box<dyn Operator>>,
) -> Value {
    unsafe { EVALUATIONS += 1 };

    match expr {
        Expression::Value { value } => value.clone(),
        Expression::Word { name } => scope
            .get(name.as_ref())
            .unwrap_or_else(|| panic!("Undefined binding: {name}"))
            .clone(),
        Expression::Operation { name, operands } => {
            // Get operation's name
            let name = name.as_ref();

            // Fetch operation
            let operator = builtins
                .get(name)
                .expect(format!("Undefined special form: {}", name).as_ref());

            operator.evaluate(operands, scope, builtins)
        }
    }
}
