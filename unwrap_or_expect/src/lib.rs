pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => return server.unwrap().to_string(),
        Security::Message => return server.expect("ERROR: program stops").to_string(),
        Security::Warning => match server {
            Ok(message) => return message.to_string(),
            Err(_) => return "WARNING: check the server".to_string(),
        },
        Security::NotFound => match server {
            Ok(message) => return message.to_string(),
            Err(message) => return format!("Not found: {}",message),
        },
        Security::UnexpectedUrl => match server {
            Ok(message) => panic!("{}",message),
            Err(message) => return message.to_string(),
        }
    }
}