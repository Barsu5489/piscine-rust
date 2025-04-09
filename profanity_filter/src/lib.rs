pub struct Message {
    pub content: String,
    pub user: String,
}

impl Message {
    pub fn new(m: &str) -> Message {
        Self {
            content: String::from(m),
            user: "barsulai".to_string(),
        }
    }

    pub fn send_ms(self) -> Option<String> {
        if self.content.contains("stupid") || self.content.is_empty() {
            None  // Reject bad messages
        } else {
            Some(self.content)  // Accept valid messages
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    let barsulai = Message::new(message);
    match barsulai.send_ms() {
        Some(_) => Ok(message), // Return message if valid
        None => Err("ERROR: illegal"), // Reject if profanity detected
    }
}





