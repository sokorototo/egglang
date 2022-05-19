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

// BUG: Values should not always be cloned everywhere, plus use string slices instead of heap strings here
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Isize(isize),
    String(String),
}
