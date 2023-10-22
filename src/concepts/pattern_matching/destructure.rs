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