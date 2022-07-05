use std::fs::read_to_string;

mod evaluator;
mod expression;
mod operators;
mod parser;
mod scope;

fn main() {
    // Read data
    let code = {
        let args = std::env::args().collect::<Vec<_>>();
        let path = args
            .get(1)
            .expect("Please provide a path to read code from");

        read_to_string(path).unwrap()
    };

    // Parse the expression
    let exprs = parser::parse(code);

    // Define runtime variables
    let mut scope = scope::build_default_scope();
    let builtins = operators::builtins();

    let result = exprs
        .iter()
        .map(|expr| evaluator::evaluate(expr, &mut scope, &builtins))
        .last()
        .unwrap_or(expression::Value::Nil);

    println!("Result of evaluation: {result:?} ");
}
