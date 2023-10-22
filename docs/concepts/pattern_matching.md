Pattern matching in Rust is a powerful feature that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. It's particularly useful with enums and structs as it enables you to write clear and concise code that can handle different kinds of data in a type-safe way.

### Pattern Matching with Enums

Enums in Rust can contain data, and pattern matching is often used to extract and work with this data. Here's an example:

```rust
// Define an enum to represent different kinds of messages.
enum Message {
    Quit,                       // No data associated
    Write(String),              // Contains a String
    ChangeColor(i32, i32, i32), // Contains three i32 values
}

fn main() {
    // Create different kinds of messages
    let messages = vec![
        Message::Quit,
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
    ];

    for message in messages {
        // Pattern match to handle each kind of message differently
        match message {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Write(text) => {
                println!("Text message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b);
            }
        }
    }
}
```

In this example, we define an enum `Message` with different variants, each potentially carrying different data. The `match` expression is then used to handle each variant differently, extracting the associated data in the process.

### Pattern Matching with Structs

Pattern matching can also be used with structs to destructure them and access their fields. Here's an example:

```rust
// Define a simple struct representing a point in 2D space.
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 0, y: 7 };

    // Pattern match to destructure and extract values from the struct.
    match point {
        Point { x, y } => {
            println!("The point is at ({}, {})", x, y);
        }
    }

    // You can also use pattern matching to destructure only some fields and ignore others
    match point {
        Point { x, .. } => {
            println!("The point is on the x-axis at {}", x);
        }
    }
}
```

In this example, we create a `Point` struct and use a `match` expression to destructure it, allowing us to work with its individual fields. The `..` syntax in the second `match` arm is used to ignore the fields we're not interested in.

### Combining Enums and Structs with Pattern Matching

You can make your code even more expressive by combining enums, structs, and pattern matching. Here's an example:

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn main() {
    let shapes = vec![
        Shape::Circle { radius: 1.0 },
        Shape::Rectangle { width: 4.0, height: 2.0 },
    ];

    for shape in shapes {
        match shape {
            Shape::Circle { radius } => println!("Circle with radius: {}", radius),
            Shape::Rectangle { width, height } => {
                println!("Rectangle with width: {} and height: {}", width, height)
            }
        }
    }
}
```

In this example, we define an enum `Shape` with variants that are structs. We then iterate over a collection of shapes, using pattern matching to destructure and handle each shape type differently.

### Conclusion

Pattern matching is one of Rust's most powerful features, allowing for highly expressive and readable code, especially when working with enums and structs. It makes it easy to handle different data types and structures, enhancing the robustness and maintainability of your code. By combining pattern matching with enums and structs, you can handle complex data types and various cases in a clear and concise manner.