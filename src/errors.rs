use std::rc::Rc;
use crate::expression::Value;

/// Result type used in this crate.
pub type EggResult<T = ()> = Result<T, EggError>;

/// Error type for egg
#[derive(Clone, Debug, thiserror::Error)]
pub enum EggError {
	#[error("Binding not found in current scope: {0}")]
	UndefinedBinding(String),
	#[error("No special form with the identifier ({0}) was found")]
	SpecialFormNotFound(String),
	#[error("Unbalanced bracket found @ location {0}")]
	UnbalancedBrackets(usize),
	#[error("Unable to parse string as number: {0}")]
	UnableToParseNumber(#[from] std::num::ParseIntError),
	#[error("Operator Complaint: {0}")]
	OperatorComplaint(String),
	#[error("No map found with the identifier: {0:?}")]
	MapNotFound(Rc<str>),
	#[error("The provided map tag: {0}, is invalid. Reason {1}")]
	InvalidMapTag(Value, String),
	#[error("{0}")]
	ParserError(String),
	#[error("Unknown Token found in TokenStream")]
	UnknownToken,
}
