use crate::{
    expression::{Expression, Value},
    special::SpecialForm,
};
use std::{collections::HashMap, rc::Rc, sync::Mutex};

pub fn evaluate<'a>(
    expr: &'a Expression,
    scope: &Mutex<HashMap<Rc<str>, Value>>,
    special_forms: *mut HashMap<&'a str, Box<dyn SpecialForm<'a> + 'a>>,
) -> Value {
    match expr {
        Expression::Value { value } => value.clone(),
        Expression::Word { name } => scope
            .lock()
            .unwrap()
            .get(name.as_str())
            .unwrap_or_else(|| panic!("Undefined binding: {name}"))
            .clone(),
        Expression::Apply { operator, operands } => {
            if let Expression::Word { name } = operator.as_ref() {
                // UNSAFE: Forced shared mutability, never runs in two threads. So no data races, also Egg can do whatever it wants whenever
                let form = unsafe { &mut *special_forms };

                let application = form.get(name.as_str()).expect("Undefined operator!");
                application.evaluate(operands, scope, unsafe { &mut *special_forms })
            } else {
                panic!("Can only call operators based on operator")
            }
        }
    }
}
