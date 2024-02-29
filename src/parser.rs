use logos::Logos;

use crate::{
	errors::{EggError, EggResult},
	expression::{Expression, Value},
};

#[derive(logos::Logos, Debug, PartialEq)]
enum Token {
	#[regex("\"([^\"\n])*\"")]
	String,
	#[regex(r"-?\d+", priority = 2)]
	Number,
	#[regex(r#"[^\s\(\),#"]+"#, priority = 1)]
	Word,

	#[regex(r"\(")]
	LeftBracket,
	#[regex(r"\)")]
	RightBracket,

	#[regex(",", logos::skip)]
	Comma,
	#[regex(r"\s+", logos::skip)]
	WhiteSpace,
	#[regex(r"#.*(\s)*", logos::skip)]
	Comment,
}

/// Parse a string into a list of expression, then calls `discover` with each expression
pub fn parse<S: AsRef<str>>(script: S) -> EggResult<Vec<Expression>> {
	let mut lex = Token::lexer(script.as_ref());

	let mut exprs = Vec::with_capacity(64);
	let mut stack = Vec::with_capacity(16);

	while let Some(token) = lex.next() {
		let token = token.map_err(|_| EggError::UnknownToken)?;
		parse_token(&token, lex.slice(), &mut exprs, &mut stack)?;
	}

	Ok(exprs)
}

fn parse_token(token: &Token, data: &str, exprs: &mut Vec<Expression>, stack: &mut Vec<usize>) -> EggResult<()> {
	match token {
		Token::String => exprs.push(Expression::Value {
			value: Value::String(data[1..data.len() - 1].into()),
		}),
		Token::Number => exprs.push(Expression::Value {
			value: Value::Number(data.parse().unwrap()),
		}),
		Token::Word => exprs.push(Expression::Word { name: (*data).into() }),

		Token::LeftBracket => stack.push(exprs.len()),
		Token::RightBracket => {
			let start = stack.pop().ok_or(EggError::UnbalancedBrackets(stack.len()))?;
			let end = exprs.len();

			// Collect operation arguments
			let parameters = exprs.drain(start..end).collect();

			// Get name of operation
			let name = exprs.pop().ok_or(EggError::UnbalancedBrackets(stack.len()))?;
			let operation = Expression::Operation {
				name: match name {
					Expression::Word { name } => name.clone(),
					_ => return Err(EggError::ParserError("Cannot use non-word as operation name".into())),
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
