# Basics

## Running Code
Print hello world to console
```rust
fn main() {
    println!("Hello World"); //Creates executable file SEMI COLONS ARE REQUIRED IN RUST
}
```

```bash
rustrc hello.rs
./hello  # Runs executable file
```

## If
```rust
fn main() {
    let number = 4; // A sample variable holding a value

    // An if-else statement that checks if 'number' is greater than 5
    if number > 5 {
        println!("The number is greater than 5");
    } else if number == 5 {
        println!("The number is equal to 5");
    } else {
        println!("The number is less than 5");
    }
}
```

## Loops
```rust
fn main() {
    // Define a vector of integers
    let numbers = vec![10, 20, 30, 40, 50];

    // Iterate over each element in the vector
    for number in numbers {
        println!("The number is {}", number);
    }
}
```