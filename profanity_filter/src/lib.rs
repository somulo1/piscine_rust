pub struct Message {
    content: String,
}

impl Message {
    // Initialize the Message structure
    pub fn new(content: &str) -> Self {
        Message {
            content: content.to_string(),
        }
    }

    // Check if the message contains "stupid" or is empty
    pub fn send_ms(self) -> Option<String> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(self.content) // Return the message content
        }
    }
}

// Function to check if a message is valid
pub fn check_ms(content: &str) -> Result<String, &str> {
    let msg = Message::new(content);
    match msg.send_ms() {
        Some(valid_content) => Ok(valid_content),
        None => Err("ERROR: illegal"),
    }
}

// Main function to test different cases
// fn main() {
//     // Test cases: use `.iter()` to avoid Rust 2021 edition warning
//     ["hello there", "", "you are stupid", "stupid"]
//         .iter()  // Use `.iter()` instead of `.into_iter()`
//         .for_each(|m| println!("{:?}", check_ms(m)));
// }
