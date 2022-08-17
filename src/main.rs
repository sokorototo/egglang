pub mod evaluator;
pub mod expression;
pub mod operators;
pub mod parser;
pub mod scope;

fn time<T>(description: &str, f: impl FnOnce() -> T) -> T {
    let timer = std::time::Instant::now();
    let res = f();

    let elapsed = timer.elapsed().as_micros();
    println!("{} took {}us", description, elapsed);
    res
}

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

    time(format!("{path}").as_str(), || {
        exprs
            .iter()
            .map(|expr| evaluator::evaluate(expr, &mut scope, &builtins))
            .for_each(drop)
    });
}
