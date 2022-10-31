pub mod errors;
pub mod evaluator;
pub mod expression;
pub mod operators;
pub mod parser;
pub mod scope;

fn main() -> errors::EggResult {
    // Read data
    let (code, path) = {
        use std::fs::read_to_string;

        let args = std::env::args().into_iter().nth(1);
        let path = args.expect("Please provide a path to read code from");

        (read_to_string(&path).unwrap(), path)
    };

    // Parse the expression
    let exprs = parser::parse(code).unwrap();

    // Define runtime variables
    let mut scope = scope::new();
    let builtins = operators::builtins();

    {
        let timer = std::time::Instant::now();
        exprs.iter().for_each(|expr| {
            evaluator::evaluate(expr, &mut scope, &builtins).unwrap();
        });

        println!(
            "{} took {}us; Evaluations: {}",
            path,
            timer.elapsed().as_micros(),
            unsafe { evaluator::EVALUATIONS }
        );
    };

    Ok(())
}
    