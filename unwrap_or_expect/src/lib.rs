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
        // Ok case: Return the server URL for all variants
        (Ok(url), Security::Unknown) => url.to_string(),
        (Ok(url), Security::Message) => url.to_string(),
        (Ok(url), Security::Warning) => url.to_string(),
        (Ok(url), Security::NotFound) => url.to_string(),
        (Ok(url), Security::UnexpectedUrl) => url.to_string(),

        // Err cases: Handle based on security_level
        (Err(_), Security::Unknown) => panic!("No server URL provided"),
        (Err(_), Security::Message) => panic!("ERROR: program stops"),
        (Err(_), Security::Warning) => String::from("WARNING: check the server"),
        (Err(msg), Security::NotFound) => format!("Not found: {}", msg),
        (Err(msg), Security::UnexpectedUrl) => panic!("{}", msg),
    }
}

fn main() {
    println!("{}", fetch_data(Ok("server1.com"), Security::Warning));
    println!("{}", fetch_data(Err("server.com"), Security::Warning));
    println!("{}", fetch_data(Err("server2.com"), Security::NotFound));

    // Panics with no custom message
    // fetch_data(Err("ERROR CRITICAL"), Security::Unknown);

    // Panics with the message "ERROR: program stops"
    // fetch_data(Err("server.com"), Security::Message);

    // Panics with the message "malicious_server.com"
    // fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
}
