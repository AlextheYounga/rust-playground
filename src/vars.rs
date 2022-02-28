// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language.

pub fn run() {
	let name = "Alex"; // Must have double quotes.
	let mut age = 26; //Make variable immutable
	age = 27;
	println!("My name is {} and I am {}", name, age);

	// Define constant
	const ID: i32 = 001;
	println!("ID: {}", ID);

	// Assign multiple
	let (my_name, my_age) = ("Alex", 26);
}