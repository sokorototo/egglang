<h4 align=center>A Rust implementation of the Egg Programming Language</h4>
<p align=center>
  <a href="https://crates.io/crates/egglang"><img alt="Crate Version on Crates.io" src="https://img.shields.io/crates/v/egglang?style=flat-square"></a>
  <a href="https://docs.rs/egglang"><img alt="docs.rs" src="https://img.shields.io/docsrs/egglang?style=flat-square"></a>
  <br/>
  <a href="https://github.com/sokorototo/vach/blob/main/LICENSE"><img alt="GitHub" src="https://img.shields.io/github/license/sokorototo/egglang?style=flat-square"></a>
  <a href="https://github.com/sokorototo/egglang/issues"><img alt="GitHub issues" src="https://img.shields.io/github/issues-raw/sokorototo/egglang?style=flat-square"></a>
</p>
<p align=center>
 <a href="https://docs.rs/egglang">Docs</a> | <a href="https://github.com/sokorototo/egglang">Repo</a>
</p>


##### `egg` is a toy Programming Language from the book Eloquest Javascript by Marijn Haverbeke, [Chapter 12](https://eloquentjavascript.net/12_language.html). The Book was pivotal to my early Programming journey. I moved to Rust some time back and in a fit of nostalgia I decided to rewrite `egg` in Rust.



### ✨ Features

- **Extensive** and **Modular** standard library; `Core`, `Objects`, `StringTools`, `Console` and `Functions`
- **Effective Scope Chain**: Local Variables and Global Variables work as expected.
- **User-Defined Functions**: Create functions in Egg using the `fn` keyword.
- **Higher Order Functions**: Pass functions as values to other functions or to built-in `Operators`.
- **Extensible**: Create your own builtin functions by implementing the [`Operator`](https://docs.rs/egglang/latest/egglang/operators/trait.Operator.html) trait.
- **no_std**: Only depends on `alloc`. Enabling the `std` feature adds the `Print`, `PrintLine`, `ReadLine`  and `Sleep` builtins.



### 🏋️‍♂️ Examples

##### To start executing a script, we need to first `parse` it, create a `Scope` and assemble a map of builtin functions it can access:

```rust
use egglang::prelude::*;

// Create the default Scope, with necessary constants set
let mut scope = Scope::default();

// Create a minimal set of operators
let mut operators = operators::empty();
operators::minimal(&mut operators);

// Parse a Script into a list of expressions
let script = "sum(12.5, 12.5, 25)";
let expressions = parse(script).unwrap();

// Evaluate the expression
let expression = &expressions[0]; // the call to `sum`
let result = evaluate(expression, &mut scope, &operators).unwrap();

assert_eq!(result, 50f32.into());
```

##### We can also define custom built-in functions by implementing `Operator` on a type:

```rust
use egglang::prelude::*;
use std::collections::BTreeMap;

// Create the default Scope, with necessary constants set
let mut scope = Scope::default();

// Insert base operators, and add console functions; Adds println
let mut operators = operators::empty();
operators::minimal(&mut operators);
operators::console(&mut operators);

// Define a `random(...)` builtin
struct Random;

impl Operator for Random {
    	fn evaluate(&self, _: &[Expression], _: &mut Scope, _: &BTreeMap<&str, Box<dyn Operator>>) -> EggResult<Value> {
    		// totally random value ;)
    		Ok(0.15.into())
    }
}

// Insert `random(...)` into Operators map
operators.insert("random", Box::new(Random));

// Parse a Script into a list of expressions
let script = r#"
define(iterations, random(0, 120))
repeat(iterations, println("oi oi oi oi oi"))
"#;
let expressions = parse(script).unwrap();

// Evaluate the expressions; define -> repeat -> ...
for expression in expressions {
  let _ = evaluate(&expression, &mut scope, &operators).unwrap();
}
```

> Example Egg scripts can be found in the `scripts` directory;

<p style="text-align: center">
	<img src="https://i.redd.it/hlgxxtijupyc1.jpeg">
	<h6 style="text-align: center">Credit: <a href="https://new.reddit.com/r/Art/comments/1cl84vd/%E9%99%85_j%C3%AC_the_frontier_upenguinwarrior3105_chalk_on/">u/Penguin-warrior-3105</a></h6>
</p>
