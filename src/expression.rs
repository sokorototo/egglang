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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Nil,
    Number(isize),
    String(Rc<str>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "\"{s}\""),
        }
    }
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
