use logos::Logos;

use crate::expression::{Expression, Value};

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

pub fn parse<S: AsRef<str>>(script: S) -> Vec<Expression> {
    let source = script.as_ref();
    let tokens = tokenize(source);
    let expressions = parse_tokens(tokens.as_slice());

    expressions
}

fn tokenize(code: &str) -> Vec<(Token, String)> {
    let mut lex = Token::lexer(code);
    let mut tokens = Vec::new();

    while let Some(token) = lex.next() {
        tokens.push((token, lex.slice().to_string()))
    }

    tokens
}

fn parse_tokens(tokens: &[(Token, String)]) -> Vec<Expression> {
    let mut expressions = Vec::new();
    let mut stack = vec![];

    // Iterate over tokens
    tokens.iter().for_each(|(token, data)| match token {
        Token::String => {
            let str = data.as_str();
            expressions.push(Expression::Value {
                value: Value::String(str[1..str.len() - 1].into()),
            })
        }
        Token::Number => expressions.push(Expression::Value {
            value: Value::Number(data.parse().expect("Unable to parse string as number")),
        }),
        Token::Word => expressions.push(Expression::Word {
            name: data.as_str().into(),
        }),

        Token::LeftBracket => stack.push(expressions.len()),
        Token::RightBracket => {
            let start = stack.pop().expect("Unbalanced brackets");
            let end = expressions.len();
            let count = end - start;

            // Collect operation arguments
            let mut sub_expressions = Vec::with_capacity(count);

            (start..end).for_each(|i| {
                let expression = expressions.get(i).expect("Unbalanced braces");
                sub_expressions.push(expression.clone());
            });

            // Truncate original expressions
            expressions.truncate(expressions.len() - count);

            // Get name of operation
            let name = expressions.pop().expect("Unbalanced brackets");
            let operation = Expression::Operation {
                name: match name {
                    Expression::Word { name } => name.clone(),
                    _ => panic!("cannot invoke non-word types"),
                },
                operands: sub_expressions,
            };

            // Push operation to stack
            expressions.push(operation);
        }

        Token::UnknownToken => panic!("Experienced unknown token in token stream: {data}"),
        _ => unreachable!("Other tokens are automatically filtered out by logos"),
    });

    expressions
}
