mod evaluator;
mod expression;
mod parser;
mod scope;
mod special;

fn main() {
    let regexi = [
        regex::Regex::new("^\"([^\"]*)\"").unwrap(),
        regex::Regex::new(r"^\d+\b").unwrap(),
        regex::Regex::new(r#"^[^\s\(\),#"]+"#).unwrap(),
    ];

    let code = r#"do(
        define(x, 10),
        if(
            0,
            print(x),
            print("small")
        ),
        repeat(10, print("We are done here"))
    )"#;

    // Parse the expression
    let expr = parser::parse(code, &regexi);

    // Define runtime variables
    let scope = scope::build_default_scope();
    let special_forms = special::build_special_forms();

    evaluator::evaluate(&expr, &scope, &special_forms);
}
