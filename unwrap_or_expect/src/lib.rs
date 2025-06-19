pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    let mut err = String::new();
    match security_level {
        Security::Unknown => server.unwrap().to_string(),
        Security::Message => server.expect("ERROR: program stops").to_string(),
        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),
        Security::NotFound => server
            .unwrap_or_else(|a| {
                err = "Not found: ".to_string() + a;
                &err
            })
            .to_string(),
        Security::UnexpectedUrl => server.unwrap_err().to_string(),
    }
}
