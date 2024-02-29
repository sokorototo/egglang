use std::sync::Arc;

/// An expression is the smallest unit of code in egg.
#[derive(Debug, Clone)]
pub enum Expression {
	Value { value: Value },
	Word { name: Arc<str> },
	Operation { name: Arc<str>, parameters: Vec<Expression> },
}

/// A value is the smallest unit of data in egg.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Value {
	Nil,
	Number(isize),
	Boolean(bool),
	String(Arc<str>),
}

impl std::fmt::Debug for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Nil => write!(f, "Nil"),
			Self::Number(arg0) => arg0.fmt(f),
			Self::String(arg0) => arg0.fmt(f),
			Self::Boolean(arg0) => arg0.fmt(f),
		}
	}
}

impl std::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Nil => write!(f, "Nil"),
			Self::Number(n) => write!(f, "{n}"),
			Self::String(s) => write!(f, "\"{s}\""),
			Self::Boolean(b) => {
				write!(f, "{}", if *b { "True" } else { "False" })
			}
		}
	}
}

impl From<bool> for Value {
	fn from(val: bool) -> Self {
		Value::Boolean(val)
	}
}

impl From<String> for Value {
	fn from(val: String) -> Self {
		Value::String(val.into())
	}
}

impl From<&str> for Value {
	fn from(val: &str) -> Self {
		Value::String(val.into())
	}
}

impl From<isize> for Value {
	fn from(val: isize) -> Self {
		Value::Number(val)
	}
}

impl From<Arc<str>> for Value {
	fn from(val: Arc<str>) -> Self {
		Value::String(val)
	}
}

impl From<()> for Value {
	fn from(_: ()) -> Self {
		Value::Nil
	}
}
