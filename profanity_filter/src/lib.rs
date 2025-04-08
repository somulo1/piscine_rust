// src/lib.rs

pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(content: &str, user: &str) -> Self {
        Message {
            content: content.to_string(),
            user: user.to_string(),
        }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    let msg = Message::new(message, "anonymous"); // user isn't important here
    match msg.send_ms() {
        Some(content) => Ok(content),
        None => Err("ERROR: illegal"),
    }
}
