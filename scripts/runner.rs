use collections::BTreeMap;
use egglang::{evaluator, operators, parser};
use std::*;

fn main() {
	match env::args().nth(1) {
		Some(path) => {
			let file = fs::read_to_string(path).unwrap();
			let expressions = parser::parse(&file).unwrap();

			let mut scope = Default::default();
			let mut operators = BTreeMap::new();
			operators::full(&mut operators);

			for expression in expressions {
				evaluator::evaluate(&expression, &mut scope, &mut operators).unwrap();
			}
		}
		None => eprintln!("No script path provided"),
	}
}
