//You can make your code even more expressive by combining enums, structs, and pattern matching. 
// Here's an example:

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