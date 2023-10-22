// Define a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello, world!");
    };
}

fn main() {
    // Use the macro.
    say_hello!();
}