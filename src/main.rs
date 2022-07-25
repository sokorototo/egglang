mod evaluator;
mod expression;
mod operators;
mod parser;
mod scope;

fn time<T>(description: &str, f: impl FnOnce() -> T) -> T {
    let timer = std::time::Instant::now();
    let res = f();

    let elapsed = timer.elapsed().as_micros();
    println!("{} took {}us", description, elapsed);
    res
}

fn main() {
    // Read data
    let code = {
        use std::fs::read_to_string;

        let args = std::env::args().into_iter().skip(1).next();
        let path = args.expect("Please provide a path to read code from");

        read_to_string(path).unwrap()
    };

    // Parse the expression
    // A -> 14153535ns
    // B -> 14153535ns
    let exprs = time("Call to parser::parse(---)", || parser::parse(code));

    // Define runtime variables
    let mut scope = scope::build_default_scope();
    let builtins = operators::builtins();

    for expr in exprs.as_slice() {
        evaluator::evaluate(expr, &mut scope, &builtins);
    }
}
