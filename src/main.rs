use std::fs::read_to_string;

mod evaluator;
mod expression;
mod parser;
mod scope;
mod special;

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
    let special_forms = special::build_special_forms();

    let result = evaluator::evaluate(&exprs[0], &mut scope, &special_forms);

    println!("Result of evaluation: {result:?} ");
}
