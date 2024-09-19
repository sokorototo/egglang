use alloc::{boxed::Box, collections::btree_map::BTreeMap, string::ToString, vec::Vec};
use core::ops::Range;
use logos::Logos;

use crate::{
	error::{EggError, EggResult},
	expression::{Expression, Value},
	operators::Operator,
};

#[derive(logos::Logos, Debug, PartialEq)]
enum Token {
	#[regex("\"([^\"\n])*\"")]
	String,
	#[regex(r"-?\d+(\.\d+)?([eE]-?\d+)?")]
	Float,
	#[regex(r#"[\p{L}_][\p{L}\d_.]*"#)]
	Word,
	#[regex("True|False")]
	Boolean,

	#[regex(r"\(")]
	LeftBracket,
	#[regex(r"\)")]
	RightBracket,

	#[regex(",", logos::skip)]
	Comma,
	#[regex(r"\s+", logos::skip)]
	WhiteSpace,
	#[regex(r"#.*(\s)+", logos::skip)]
	Comment,
}

/// Given a valid script: `S`, parses and returns a vector of [`Expressions`](Expression).
/// Iterate over result and execute each expression using [`evaluate`](crate::evaluator::evaluate).
///
/// See [`evaluate`](crate::evaluator::evaluate) docs for sample usage.
///
/// > IMPORTANT: Mutating `operators` after Expression is generated is undefined behaviour, as Expression holds several pointers to the data in `operators`
pub fn parse<S: AsRef<str>>(script: S, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Vec<Expression>> {
	let script = script.as_ref();
	let lex = Token::lexer(script);

	let mut exprs = Vec::with_capacity(64);
	let mut stack = Vec::with_capacity(16);

	for (token, span) in lex.spanned() {
		let token = token.map_err(|_| EggError::UnknownToken(script[span.clone()].to_string()))?;
		parse_token(&token, script, span, &mut exprs, &mut stack, operators)?;
	}

	exprs.shrink_to_fit();
	Ok(exprs)
}

fn parse_token(token: &Token, source: &str, span: Range<usize>, exprs: &mut Vec<Expression>, stack: &mut Vec<usize>, operators: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<()> {
	let data = &source[span.clone()];

	match token {
		Token::Boolean => exprs.push(Expression::Value {
			value: Value::Boolean(data == "True"),
		}),
		Token::String => exprs.push(Expression::Value {
			value: Value::String(data[1..data.len() - 1].into()),
		}),
		Token::Float => exprs.push(Expression::Value {
			value: Value::Number(data.parse().unwrap()),
		}),
		Token::Word => exprs.push(Expression::Word { name: data.into() }),

		Token::LeftBracket => stack.push(exprs.len()),
		Token::RightBracket => {
			let start = stack.pop().ok_or(EggError::UnbalancedBrackets(span.start))?;
			let end = exprs.len();

			// Collect operation arguments
			let parameters = exprs.drain(start..end).collect();

			// Get name of operation
			let name = exprs.pop().ok_or(EggError::UnbalancedBrackets(span.start))?;
			let operation = Expression::FnCall {
				identifier: match name {
					Expression::Word { name } => match operators.get(name.as_str()) {
						Some(op) => either::Right(op.as_ref() as _),
						None => either::Either::Left(name.clone()),
					},
					_ => return Err(EggError::ParserError(span, "Cannot use non-word as operation name".into())),
				},
				parameters,
			};

			// Push operation to stack
			exprs.push(operation);
		}

		_ => unreachable!("Other tokens are automatically filtered out by logos"),
	};

	Ok(())
}
