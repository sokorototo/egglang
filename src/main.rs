pub mod errors;
pub mod evaluator;
pub mod expression;
pub mod operators;
pub mod parser;
pub mod scope;

fn main() {
    // Read data
    let code = std::env::args()
        .into_iter()
        .nth(1)
        .map(std::fs::read_to_string)
        .unwrap()
        .expect("Please provide a code path");

    // Parse the expression
    let exprs = parser::parse(code).unwrap();

    // Define runtime variables
    let mut scope = scope::new();
    let builtins = operators::builtins();

    exprs.iter().for_each(|expr| {
        evaluator::evaluate(expr, &mut scope, &builtins).unwrap();
    })
}
