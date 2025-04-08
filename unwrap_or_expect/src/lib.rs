pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match  server {
        Ok(url) =>  {
            match security_level {
                Security::UnexpectedUrl => panic!("{url}"),
                _ => format!("{url}"),
            }
    },
        Err(e) => {
            match security_level {
                Security::Unknown => format!("{}",server.unwrap()),
                Security::Message => server.expect("ERROR: program stops").to_string(),
                Security::Warning => format!("WARNING: check the server"),
                Security::NotFound => format!("Not found: {e}"),
                Security::UnexpectedUrl => format!("{e}"),
            }
        }
    }
}

