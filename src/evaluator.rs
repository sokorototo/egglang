use crate::{
    expression::{Expression, Value},
    special::SpecialForm,
};
use std::{collections::HashMap, sync::Mutex};

pub fn evaluate<'a>(
    expr: &Expression,
    scope: &Mutex<HashMap<String, Value>>,
    special_forms: *mut HashMap<&'a str, Box<dyn SpecialForm<'a> + 'a>>,
) -> Value {
    match expr {
        Expression::Value { value } => value.clone(),
        Expression::Word { name } => scope
            .lock()
            .unwrap()
            .get(name.as_str())
            .expect(format!("Undefined binding: {name}").as_str())
            .clone(),
        Expression::Apply { operator, operands } => {
            if let Expression::Word { name } = operator.as_ref() {
                let form = unsafe { &mut *special_forms };
                let application = form.get(name.as_str()).expect("Undefined operator!");
                return application.evaluate(operands, scope, unsafe { &mut *special_forms });
            } else {
                panic!("Can only call operators based on operator")
            }
        }
    }
}
