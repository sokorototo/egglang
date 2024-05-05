use crate::{evaluator, operators, parser};
use std::fs::{read_dir, read_to_string};

#[test]
fn test() {
	// Define runtime variables
	let mut operators = operators::empty();
	operators::full(&mut operators);

	// Read data
	let scripts = {
		read_dir("scripts")
			.unwrap()
			.filter_map(|entry| entry.ok())
			.filter(|entry| entry.file_type().ok().map(|t| t.is_file()).unwrap_or(false))
			.filter_map(|entry| read_to_string(entry.path()).ok().map(|s| (entry.path(), s)))
	};

	scripts.for_each(|(path, script)| {
		let mut scope = Default::default();

		println!("\n[{}]", path.display());
		let ast = parser::parse(script).unwrap();

		ast.iter().for_each(|expr| {
			evaluator::evaluate(expr, &mut scope, &operators).unwrap();
		});
	});

	println!("\n[Calls to evaluate(...)] {}", unsafe { evaluator::EVALUATIONS });
}
