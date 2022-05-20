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
        define(x, "11"),
        define(res, 20),

        # Basic conditional
        if(
            equals(typeof(x), NUMBER),
            if(
                greater_than(x, 10),
                print_line("x is a number and is greater than 10"),
                print_line("x is a number, but is not greater than 10")
            ),
            print_line("x is not a number")
        ),

        # Demonstration of repeat
        mutate(res, 0),
        repeat(10000, repeat(10000, mutate(res, multiply(res, 1)))),

        # Demonstration of println
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
