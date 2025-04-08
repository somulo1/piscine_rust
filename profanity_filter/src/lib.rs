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

pub fn check_ms(content: &str) -> Result<&str, &str> {
    let msg = Message::new(content, "user"); // user field can be anything
    match msg.send_ms() {
        Some(valid_content) => Ok(valid_content),
        None => Err("ERROR: illegal"),
    }
}
