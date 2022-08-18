pub mod evaluator;
pub mod expression;
pub mod operators;
pub mod parser;
pub mod scope;

fn main() {
    // Read data
    let (code, path) = {
        use std::fs::read_to_string;

        let args = std::env::args().into_iter().skip(1).next();
        let path = args.expect("Please provide a path to read code from");

        (read_to_string(&path).unwrap(), path)
    };

    // Parse the expression
    let exprs = parser::parse(code);

    // Define runtime variables
    let mut scope = scope::new();
    let builtins = operators::builtins();

    {
        let timer = std::time::Instant::now();

        for expr in exprs.as_slice() {
            evaluator::evaluate(&expr, &mut scope, &builtins);
        }

        println!("{} took {}us", path, timer.elapsed().as_micros());
    };
}
