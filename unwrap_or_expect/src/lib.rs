// src/lib.rs
pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),

        Security::Message => server.expect("ERROR: program stops").to_string(),

        Security::Warning => match server {
            Ok(url) => url.to_string(),
            Err(_) => "WARNING: check the server".to_string(),
        },

        Security::NotFound => match server {
            Ok(url) => url.to_string(),
            Err(err_msg) => format!("Not found: {}", err_msg),
        },

        Security::UnexpectedUrl => match server {
            Ok(url) => panic!("{}", url),
            Err(err_msg) => err_msg.to_string(),
        },
    }
}
