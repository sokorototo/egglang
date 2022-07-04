use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Expression {
    Value {
        value: Value,
    },
    Word {
        name: Rc<str>,
    },
    Operation {
        name: Rc<str>,
        operands: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Number(isize),
    String(Rc<str>),
    Nil,
}

impl From<bool> for Value {
    fn from(val: bool) -> Self {
        if val {
            Value::Number(1)
        } else {
            Value::Number(0)
        }
    }
}
