mod evaluator;
mod expression;
mod operators;
mod parser;
mod scope;

fn main() {
    // Read data
    let code = {
        use std::fs::read_to_string;

        let args = std::env::args().into_iter().skip(1).next();
        let path = args.expect("Please provide a path to read code from");

        read_to_string(path).unwrap()
    };

    // Parse the expression
    let exprs = parser::parse(code);

    // Define runtime variables
    let mut scope = scope::build_default_scope();
    let builtins = operators::builtins();

    for expr in exprs {
        evaluator::evaluate(&expr, &mut scope, &builtins);
    }
}
