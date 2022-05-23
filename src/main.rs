use std::fs::read_to_string;

mod evaluator;
mod expression;
mod parser;
mod scope;
mod special;

fn main() {
    let regexi = [
        // String
        regex::Regex::new("^\"([^\"]*)\"").unwrap(),
        // Number
        regex::Regex::new(r"^\d+\b").unwrap(),
        // Word
        regex::Regex::new(r#"^[^\s\(\),#"]+"#).unwrap(),
        // Comment
        regex::Regex::new(r"^#.*\n\s+").unwrap(),
    ];

    // Read data
    let code = {
        let args = std::env::args().collect::<Vec<_>>();
        let path = match args.get(1) {
            Some(path) => path,
            None => panic!("Please provide a path to read code from"),
        };

        read_to_string(path).unwrap()
    };

    // Parse the expression
    let expr = parser::parse(code, &regexi);

    // Define runtime variables
    let mut scope = scope::build_default_scope();
    let special_forms = special::build_special_forms();

    evaluator::evaluate(&expr, &mut scope, &special_forms);
}
