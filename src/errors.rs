use crate::expression::Value;

use alloc::string::String;
use arcstr::ArcStr;
use core::ops::Range;

/// Result type used in this crate.
pub type EggResult<T = ()> = Result<T, EggError>;

/// Error type for egg
#[derive(Clone, Debug, thiserror_no_std::Error)]
pub enum EggError {
	#[error("Binding not found in current scope: {0}")]
	UndefinedBinding(ArcStr),
	#[error("No Function with the identifier ({0}) was found")]
	FunctionNotFound(ArcStr),
	#[error("Unbalanced bracket found at location: {0}")]
	UnbalancedBrackets(usize),
	#[error("Unable to parse string as number: {0}")]
	UnableToParseNumber(#[from] core::num::ParseFloatError),
	#[error("Operator Complaint: {0}")]
	OperatorComplaint(String),
	#[error("No map found with the identifier: {0:?}")]
	MapNotFound(ArcStr),
	#[error("The provided map tag: {0}, is invalid. Reason {1}")]
	InvalidMapTag(Value, String),
	#[error("Invalid Function Definition: {0}")]
	InvalidFunctionDefinition(String),
	#[error("Assertion failed: {0}")]
	AssertionFailed(Value),
	#[error("Generic parsing error at: {0:?}. Reason: {1}")]
	ParserError(Range<usize>, String),
	#[error("Unknown Token found in TokenStream")]
	UnknownToken,
	#[error("{0}")]
	InvalidFunctionCall(String),
}
