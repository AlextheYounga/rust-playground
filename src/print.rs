pub fn run() {
	println!("Hello from print.rs file");

	// Cant directly print number, must be encased in string
	println!("Number: {}", 1);

	// Positional Arguments.
	println!(
		"{0} is from {1} and likes to {2}",
		"Alex", "New Orleans", "code"
	);

	// Named Arguments.
	println!(
		"{name} is from {location} and likes to {activity}",
		name = "Alex",
		location = "New Orleans",
		activity = "code"
	);
	// Placeholder traits
	println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

	// Placeholder for debug trait
	println!("{:?}", (12, true, "hello"));

	//Basic math
	println!("10 + 10 = {}", 10 + 10);
}
