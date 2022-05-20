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

impl Into<Value> for bool {
    fn into(self) -> Value {
        if self {
            Value::Number(1)
        } else {
            Value::Number(0)
        }
    }
}
