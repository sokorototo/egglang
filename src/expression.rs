use alloc::vec::Vec;
use arcstr::ArcStr;
use ordered_float::OrderedFloat;

/// An expression is the smallest unit of code in egg.
#[derive(Debug, Clone)]
pub enum Expression {
	Value { value: Value },
	Word { name: ArcStr },
	FnCall { name: ArcStr, parameters: Vec<Expression> },
}

/// A value is the smallest unit of data in egg.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
	Nil,
	Number(OrderedFloat<f32>),
	Boolean(bool),
	String(ArcStr),
	Function(usize),
}

impl alloc::fmt::Debug for Value {
	fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
		match self {
			Self::Nil => write!(f, "Nil"),
			Self::Number(arg0) => arg0.0.fmt(f),
			Self::String(arg0) => arg0.fmt(f),
			Self::Boolean(arg0) => arg0.fmt(f),
			Self::Function(arg0) => write!(f, "FunctionDefinition @ {}", arg0),
		}
	}
}

impl alloc::fmt::Display for Value {
	fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
		match self {
			Self::Nil => write!(f, "Nil"),
			Self::Number(n) => write!(f, "{n}"),
			Self::String(s) => write!(f, "\"{s}\""),
			Self::Boolean(b) => {
				write!(f, "{}", if *b { "True" } else { "False" })
			}
			Self::Function(_) => write!(f, "Function"),
		}
	}
}

impl From<bool> for Value {
	fn from(val: bool) -> Self {
		Value::Boolean(val)
	}
}

impl From<&str> for Value {
	fn from(val: &str) -> Self {
		Value::String(val.into())
	}
}

impl From<f32> for Value {
	fn from(val: f32) -> Self {
		Value::Number(OrderedFloat(val))
	}
}

impl From<ArcStr> for Value {
	fn from(val: ArcStr) -> Self {
		Value::String(val)
	}
}

impl From<()> for Value {
	fn from(_: ()) -> Self {
		Value::Nil
	}
}
