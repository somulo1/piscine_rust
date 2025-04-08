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

    pub fn send_ms(&self) -> Option<String> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(self.content.clone())
        }
    }
}

pub fn check_ms(message: &str) -> Result<String, String> {
    let msg = Message::new(message, "anonymous");
    match msg.send_ms() {
        Some(content) => Ok(content),
        None => Err("ERROR: illegal".to_string()),
    }
}
