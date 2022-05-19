use crate::expression::{Expression, Value};

pub fn parse(source: &str, regexi: &[regex::Regex; 3]) -> Expression {
    let (expr, res) = parse_expression(source, regexi);

    if res.trim_start().len() > 0 {
        println!("Error?");
    };

    expr
}

pub fn parse_expression<'t>(source: &'t str, regexi: &[regex::Regex; 3]) -> (Expression, &'t str) {
    let source = source.trim_start();

    // Get regex definitions
    let string_regex = &regexi[0];
    let number_regex = &regexi[1];
    let word_regex = &regexi[2];

    let (expr, len) = if let Some(captures) = string_regex.captures(source) {
        let str = &captures[1];
        let full = &captures[0];

        (
            Expression::Value {
                value: Value::String(str.to_string()),
            },
            full.len(),
        )
    } else if let Some(captures) = number_regex.captures(source) {
        let str = &captures[0];

        (
            Expression::Value {
                value: Value::Isize(str.parse().unwrap()),
            },
            str.len(),
        )
    } else if let Some(captures) = word_regex.captures(source) {
        let str = &captures[0];

        (
            Expression::Word {
                name: str.to_string(),
            },
            str.len(),
        )
    } else {
        panic!("Syntax error! Unknown syntax {}", source)
    };

    return parse_apply(expr, &source[len..], regexi);
}

pub fn parse_apply<'t>(
    expr: Expression,
    source: &'t str,
    regexi: &[regex::Regex; 3],
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
    let expr = Expression::Apply {
        operator: Box::new(expr),
        operands: args,
    };

    return parse_apply(expr, &source[1..], regexi);
}
