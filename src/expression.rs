use std::rc::Rc;

#[derive(Debug)]
pub enum Expression {
    Value {
        value: Value,
    },
    Word {
        name: Rc<str>,
    },
    Apply {
        operator: Box<Expression>,
        operands: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Number(isize),
    String(Rc<str>),
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
