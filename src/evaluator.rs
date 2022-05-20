use crate::{
    expression::{Expression, Value},
    special::SpecialForm,
};
use std::{collections::HashMap, rc::Rc, sync::Mutex};

pub fn evaluate<'a>(
    expr: &'a Expression,
    scope: &Mutex<HashMap<Rc<str>, Value>>,
    special_forms: &HashMap<&'a str, Box<dyn SpecialForm<'a> + 'a>>,
) -> Value {
    match expr {
        Expression::Value { value } => value.clone(),
        Expression::Word { name } => scope
            .lock()
            .unwrap()
            .get(name)
            .unwrap_or_else(|| panic!("Undefined binding: {name}"))
            .clone(),
        Expression::Apply { operator, operands } => {
            if let Expression::Word { name } = operator.as_ref() {
                let application = special_forms
                    .get(name.as_ref())
                    .expect("Undefined operator!");

                    application.evaluate(operands, scope, special_forms)
            } else {
                panic!("Can only call operators based on operator")
            }
        }
    }
}
