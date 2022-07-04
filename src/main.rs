use std::fs::read_to_string;

mod evaluator;
mod expression;
mod parser;
mod scope;
mod special;

// Simple timing utility
fn time<T>(tag: &str, f: impl FnOnce() -> T) -> T {
    let start = std::time::Instant::now();
    let result = f();

    println!("Execution [{tag}] took: {:?}", start.elapsed());
    result
}

fn main() {
    // Read data
    let code = {
        let args = std::env::args().collect::<Vec<_>>();
        let path = args
            .get(1)
            .expect("Please provide a path to read code from");

        read_to_string(path).unwrap()
    };

    // Defined regexi
    let regexi = &[
        // String
        regex::Regex::new("^\"([^\"]*)\"").unwrap(),
        // Number
        regex::Regex::new(r"^(\+|-)?\d+\b").unwrap(),
        // Word
        regex::Regex::new(r#"^[^\s\(\),#"]+"#).unwrap(),
        // Comment
        regex::Regex::new(r"^#.*\n\s+").unwrap(),
    ];

    // Parse the expression
    // let expr = time("Regex Tokenization", || parser::parse(&code, regexi));
    let exprs = time("Logos Tokenization", || parser::logos_parser::parse(code));

    // Define runtime variables
    let mut scope = scope::build_default_scope();
    let special_forms = special::build_special_forms();

    // Log outputs
    std::fs::write("logos.txt", format!("{:#?}", &exprs[0])).unwrap();
    // std::fs::write("regex.txt", format!("{expr:#?}")).unwrap();

    let result = time("Expression evaluation", || {
        evaluator::evaluate(&exprs[0], &mut scope, &special_forms)
    });

    println!("Result of evaluation: {result:?} ");
}
