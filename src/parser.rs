use logos::Logos;

use crate::{
    errors::{EggError, EggResult},
    expression::{Expression, Value},
};

#[derive(logos::Logos, Debug, PartialEq)]
enum Token {
    #[regex("\"([^\"\n])*\"")]
    String,
    #[regex(r"(\+|-)?\d+", priority = 2)]
    Number,
    #[regex(r#"[^\s\(\),#"]+"#)]
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

    #[error]
    UnknownToken,
}

pub fn parse<S: AsRef<str>>(script: S) -> EggResult<Vec<Expression>> {
    let source = script.as_ref();
    let tokens = tokenize(source);
    let expressions = parse_tokens(tokens.as_slice());

    expressions
}

fn tokenize(code: &str) -> Vec<(Token, &str)> {
    let mut lex = Token::lexer(code);
    let mut tokens = Vec::new();

    while let Some(token) = lex.next() {
        tokens.push((token, lex.slice()))
    }

    tokens
}

fn parse_tokens(tokens: &[(Token, &str)]) -> EggResult<Vec<Expression>> {
    let mut exprs = vec![];
    let mut stack = vec![];

    // Iterate over tokens
    for (token, data) in tokens.iter() {
        match token {
            Token::String => exprs.push(Expression::Value {
                value: Value::String(data[1..data.len() - 1].into()),
            }),
            Token::Number => exprs.push(Expression::Value {
                value: Value::Number(data.parse().unwrap()),
            }),
            Token::Word => exprs.push(Expression::Word {
                name: (*data).into(),
            }),

            Token::LeftBracket => stack.push(exprs.len()),
            Token::RightBracket => {
                let start = stack
                    .pop()
                    .ok_or(EggError::UnbalancedBrackets(stack.len()))?;
                let end = exprs.len();

                // Collect operation arguments
                let sub_expressions = exprs.drain(start..end).collect();

                // Get name of operation
                let name = exprs
                    .pop()
                    .ok_or(EggError::UnbalancedBrackets(stack.len()))?;
                let operation = Expression::Operation {
                    name: match name {
                        Expression::Word { name } => name.clone(),
                        _ => {
                            return Err(EggError::ParserError(
                                "Cannot use non-word as operation name".into(),
                            ))
                        }
                    },
                    operands: sub_expressions,
                };

                // Push operation to stack
                exprs.push(operation);
            }

            Token::UnknownToken => {
                return Err(EggError::ParserError(format!(
                    "Experienced unknown token in token stream: {data}"
                )))
            }
            _ => unreachable!("Other tokens are automatically filtered out by logos"),
        }
    }

    Ok(exprs)
}
