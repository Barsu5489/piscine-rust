pub struct Message {

   pub content: String,
   pub user: String

}

impl Message{
    pub fn new(m: &str) -> Message{
        Self {
            content: String::from(m),
            user : "barsulai".to_string(),
        }
    }

    pub fn send_ms(self : Self) -> Option<String>{
        if self.content.contains("stupid") || self.content.is_empty() {
            Some(self.content)
        } else {
            None
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    let barsulai = Message::new(message);
    match barsulai.send_ms() {
        Some(m) => Ok(message),
        None => Err("ERROR: illegal"),
    }
}




