pub fn take_ownership(some_string: String) {
    // `some_string` is passed to this function, so it is owned by this function.
    // When the function ends, `some_string` is dropped and its memory is freed.
    println!("{}", some_string);
}


pub fn calculate_length(s: &String) -> usize {
    // `s` is a reference to a String object.
    // The function does not own `s`, so it will not be dropped when the function ends.
    // The function cannot modify `s` because it is a reference, not an owned value.
    s.len()
}

fn main() {
    // This variable owns the String object in memory.
    let my_string = String::from("hello, world");

    // At this point, `my_string`'s ownership is passed to the function `take_ownership`.
    // `my_string` can no longer be used from this point in the `main` function.
    // take_ownership() is just an example function to demonstrate the concept, it is not a native Rust function.
    take_ownership(my_string);

    // The next line would throw a compile-time error because `my_string`'s value has been moved.
    // println!("{}", my_string);

    // Here, the variable `another_string` is initialized.
    let another_string = String::from("hello, Rust");

    // `another_string` is passed to `calculate_length`, but its ownership is borrowed, not taken.
    // This means we can continue using `another_string` after this function call.
    let length = calculate_length(&another_string);

    println!("The length of '{}' is {}.", another_string, length);
}