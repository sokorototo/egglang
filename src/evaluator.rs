use crate::{
    expression::{Expression, Value},
    special::SpecialForm,
};
use std::collections::HashMap;

pub fn evaluate<'a>(
    expr: &'a Expression,
    scope: &mut HashMap<String, Value>,
    special_forms: &HashMap<&'a str, Box<dyn SpecialForm<'a> + 'a>>,
) -> Value {
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
            let application = special_forms
                .get(name)
                .expect(format!("Undefined special form: {}", name).as_ref());

            application.evaluate(operands, scope, special_forms)
        }
    }
}
