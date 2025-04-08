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

    pub fn send_ms(self) -> Option<String> { // `self` is taken by value here
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(self.content)  // Return owned value instead of a reference
        }
    }
}

pub fn check_ms(content: &str) -> Result<String, &str> {  // return owned String
    let msg = Message::new(content, "user");
    match msg.send_ms() {
        Some(valid_content) => Ok(valid_content),
        None => Err("ERROR: illegal"),
    }
}
