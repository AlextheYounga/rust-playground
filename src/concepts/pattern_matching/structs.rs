// Pattern matching can also be used with structs to destructure them and access their fields.
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