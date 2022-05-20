#[derive(Debug)]
pub enum Expression {
    Value {
        value: Value,
    },
    Word {
        name: String,
    },
    Apply {
        operator: Box<Expression>,
        operands: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Isize(isize),
    String(String),
}
