use crate::{
    expression::{Expression, Value},
    special::SpecialForm,
};
use std::{collections::HashMap, sync::Mutex};

pub fn evaluate<'a>(
    expr: &'a Expression,
    scope: &'a Mutex<HashMap<&'a str, Value>>,
    special_forms: &HashMap<&'a str, Box<dyn SpecialForm<'a> + 'a>>,
) -> Value {
    match expr {
        Expression::Value { value } => value.clone(),
        Expression::Word { name } => scope
            .lock()
            .unwrap()
            .get(name.as_str())
            .expect(format!("Undefined biding: {name}").as_str())
            .clone(),
        Expression::Apply { operator, operands } => {
            if let Expression::Word { name } = operator.as_ref() {
                let form = special_forms.get(name.as_str()).unwrap();
                return form.evaluate(operands, scope, special_forms);
            } else {
                //  let op = evaluate(expr, scope, special_forms);
                todo!()
            }
        }
    }
}
