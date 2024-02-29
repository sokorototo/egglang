use crate::{evaluator, operators, parser, scope};

#[test]
fn test() {
	use std::fs::{read_dir, read_to_string};

	// Define runtime variables
	let builtins = operators::std();

	// Read data
	let scripts = {
		let script_folder = "scripts";
		read_dir(script_folder)
			.unwrap()
			.filter_map(|entry| entry.ok())
			.filter(|entry| entry.file_type().ok().map(|t| t.is_file()).unwrap_or(false))
			.filter_map(|entry| read_to_string(entry.path()).ok().map(|s| (entry.path(), s)))
	};

	scripts.for_each(|(path, script)| {
		let mut scope = scope::default();

		println!("\nRunning script: [{}]", path.display());
		let ast = parser::parse(script).unwrap();

		ast.iter().for_each(|expr| {
			evaluator::evaluate(expr, &mut scope, &builtins).unwrap();
		});
	});
}
