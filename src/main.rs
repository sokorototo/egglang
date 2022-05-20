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
        regex::Regex::new(r"^#.*\n\s+").unwrap(),
    ];

    let code = r#"do(
        # Variable definitions
        define(x, 10),
        define(x2, multiply(10, 2)),
        define(res, 0),

        # Basic conditional
        if(
            equals(typeof(x), NUMBER),
            print_line("x is a number"),
            print_line("x is not a number")
        ),

        # Demonstration of repeat
        repeat(x2, mutate(res, add(res, 5))),

        # Demonstration of repeat
        print_line(typeof(x)),
        print_line(res),

        # Memory manipulation
        delete(res),
        print_line(exists(res)),
    )"#;

    // Parse the expression
    let expr = parser::parse(code, &regexi);

    // Define runtime variables
    let scope = scope::build_default_scope();
    let mut special_forms = special::build_special_forms();

    evaluator::evaluate(&expr, &scope, (&mut special_forms) as _);
}
