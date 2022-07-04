pub mod logos_parser;
use crate::expression::{Expression, Value};

// Parse a string into an expression
pub fn parse<S: AsRef<str>>(source: S, regexi: &[regex::Regex; 4]) -> Expression {
    let (expr, _) = parse_expression(source.as_ref(), regexi);
    expr
}

// Given an expression, returns the token type
fn parse_expression<'t>(source: &'t str, regexi: &[regex::Regex; 4]) -> (Expression, &'t str) {
    let mut source = source.trim_start();

    // Get regex definitions
    let string_regex = &regexi[0];
    let number_regex = &regexi[1];
    let word_regex = &regexi[2];

    // Trims comments
    let comment_regex = &regexi[3];
    while let Some(captures) = comment_regex.captures(source) {
        source = &source[captures[0].len()..];
    }

    let (expr, len) = if let Some(captures) = string_regex.captures(source) {
        let str = &captures[1];
        let full = &captures[0];

        (
            Expression::Value {
                value: Value::String(str.into()),
            },
            full.len(),
        )
    } else if let Some(captures) = number_regex.captures(source) {
        let str = &captures[0];

        (
            Expression::Value {
                value: Value::Number(str.parse().unwrap()),
            },
            str.len(),
        )
    } else if let Some(captures) = word_regex.captures(source) {
        let str = &captures[0];

        (Expression::Word { name: str.into() }, str.len())
    } else {
        panic!("Syntax error! Unknown syntax {}", source)
    };

    parse_operations(expr, &source[len..], regexi)
}

// Given a stream of characters, separates characters into valid chunks to be parsed into expressions
fn parse_operations<'t>(
    expr: Expression,
    source: &'t str,
    regexi: &[regex::Regex; 4],
) -> (Expression, &'t str) {
    let source = source.trim_start();

    // Is this an application?
    if !source.starts_with('(') {
        return (expr, source);
    }

    // Remove the leading '('
    let mut source = &source[1..];

    // Gather arguments
    let mut args = vec![];

    while !source.starts_with(')') {
        let (expression, rest) = parse_expression(source, regexi);

        args.push(expression);
        source = rest.trim_start();

        // trim ',' get second param
        if source.starts_with(',') {
            source = source[1..].trim();
        } else if !source.starts_with(')') {
            dbg!(source);
            panic!("Expected a ',' or a ')'")
        }
    }

    // Build new application
    let expr = Expression::Operation {
        operator: Box::new(expr),
        operands: args,
    };

    parse_operations(expr, &source[1..], regexi)
}
