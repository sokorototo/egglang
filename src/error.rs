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
	#[error("Operator Complaint: {0}")]
	OperatorComplaint(String),
	#[error("Only primitives can be used as keys in objects, found: {0}")]
	InvalidObjectKey(Value),
	#[error("The provided Value: {0} is not an Object Reference")]
	InvalidObjectReference(Value),
	#[error("Invalid Function Definition: {0}")]
	InvalidFunctionDefinition(String),
	#[error("Assertion failed: {0}")]
	AssertionFailed(Value),
	#[error("Generic parsing error at: {0:?}. Reason: {1}")]
	ParserError(Range<usize>, String),
	#[error("Unknown Token found in TokenStream: {0}")]
	UnknownToken(String),
	#[error("{0}")]
	InvalidFunctionCall(String),
}
