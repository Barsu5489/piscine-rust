pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}


// Implement fetch_data with Result<&str, &str>

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match (server, security_level) {
        (Ok(url), Security::Unknown) => url.to_string(),
        (Err(_), Security::Unknown) => panic!(), // Panics with no message

        (Ok(url), Security::Message) => url.to_string(),
        (Err(_), Security::Message) => panic!("ERROR: program stops"), // Panics with message

        (Ok(url), Security::Warning) => url.to_string(),
        (Err(_), Security::Warning) => "WARNING: check the server".to_string(),

        (Ok(url), Security::NotFound) => url.to_string(),
        (Err(msg), Security::NotFound) => format!("Not found: {}", msg),

        (Err(msg), Security::UnexpectedUrl) => msg.to_string(),
        (Ok(url), Security::UnexpectedUrl) => panic!("{}", url), // Panics with server URL
    }
}
