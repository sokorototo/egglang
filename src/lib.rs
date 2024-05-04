#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

#[cfg(test)]
mod tests;

pub mod errors;
pub mod evaluator;
pub mod expression;
pub mod operators;
pub mod parser;
pub mod scope;
