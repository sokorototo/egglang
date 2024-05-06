use alloc::vec::Vec;
use arcstr::ArcStr;
use ordered_float::OrderedFloat;

/// An expression is a piece of code that can be evaluated into a [`Value`].
#[derive(Debug, Clone)]
pub enum Expression {
	Value { value: Value },
	Word { name: ArcStr },
	FnCall { name: ArcStr, parameters: Vec<Expression> },
}

/// A primitive in Egg; can be a number, boolean, string, function, or an object.
/// Primitives are immutable, to mutate create a new value.
/// This applies to both user-code and built-in functions.
///
/// Some Rust primitives can be converted into a [`Value`] using the [`From`] trait.
///
/// ```rust
/// use egglang::expression::Value;
///
/// let number: Value = 42.0.into();
/// let string: Value = "Hello, World!".into();
/// let boolean: Value = true.into();
/// let nil: Value = ().into();
///
/// // Option<T: Into<Value>> can be converted into Value
/// assert_eq!(Value::Nil, None::<f32>.into());
/// let five: Value = Some(5.0).into();
/// assert_eq!(Value::from(5.0), five);
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
	/// Represents the absence of a value.
	Nil,
	/// A 32-bit floating-point number.
	Number(OrderedFloat<f32>),
	/// A boolean value.
	Boolean(bool),
	/// Atomically reference-counted string.
	String(ArcStr),
	/// A function definition. Stores a index to the function in the scope.
	Function(usize),
	/// An object. Stores a index to the object in the scope.
	Object(usize),
}

impl alloc::fmt::Debug for Value {
	fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
		match self {
			Self::Nil => write!(f, "Nil"),
			Self::Number(arg0) => arg0.0.fmt(f),
			Self::String(arg0) => arg0.fmt(f),
			Self::Boolean(arg0) => arg0.fmt(f),
			Self::Function(arg0) => write!(f, "FunctionDefinition @ {}", arg0),
			Self::Object(arg0) => write!(f, "Object @ {}", arg0),
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
			Self::Object(_) => write!(f, "Object"),
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

impl<T: Into<Value>> From<Option<T>> for Value {
	fn from(val: Option<T>) -> Self {
		match val {
			Some(val) => val.into(),
			None => Value::Nil,
		}
	}
}
