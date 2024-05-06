#![cfg_attr(not(feature = "std"), no_std)]
#[doc = include_str!("../README.md")]
extern crate alloc;

/// Easy access to the crate's most important types and functions
pub mod prelude {
	pub use crate::{
		error::{EggError, EggResult},
		evaluator::evaluate,
		expression::{Expression, Value},
		operators::{self, Operator},
		parser::parse,
		scope::Scope,
	};

	pub use arcstr::ArcStr;
	pub use ordered_float::OrderedFloat;
}

#[cfg(test)]
mod tests;

/// Error and Result types
pub mod error;
/// [`Evaluates`](expression::Expression) an expression into a [`Value`](expression::Value)
pub mod evaluator;
/// [`Expression`](expression::Expression) and [`Value`](expression::Value) types
pub mod expression;
/// Traits for defining functions in Rust callable in Egg, as well as several builtin functions
pub mod operators;
/// [`Parser`](parser::parse) for Egg scripts
pub mod parser;
/// Contains the [`Scope`](scope::Scope) struct, which stores variables and allows for creation of local scopes
pub mod scope;
