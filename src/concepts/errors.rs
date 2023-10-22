use std::fs::File;
use std::io::{self, Read};

// This function returns a `Result` type with a `String` in the `Ok` variant
// and an `io::Error` in the `Err` variant.
fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    // Try to open the file.
    let mut file = match File::open(path) {
        // If the file opens successfully, the `Ok` value is returned.
        Ok(file) => file,
        // If there's an error, it's returned immediately to the calling function.
        Err(e) => return Err(e),
    };

    let mut text = String::new();
    // Try to read the text from the file.
    match file.read_to_string(&mut text) {
        // If the reading is successful, the text is returned within an `Ok`.
        Ok(_) => Ok(text),
        // If there's an error, it's returned immediately to the calling function.
        Err(e) => Err(e),
    }
}

fn main() {
    match read_text_from_file("filename.txt") {
        Ok(text) => {
            // If the function succeeds, you'll have the text here.
            println!("File content: {}", text);
        }
        Err(e) => {
            // If the function fails, you'll handle the error here.
            println!("Failed to read the file: {}", e);
        }
    }
}