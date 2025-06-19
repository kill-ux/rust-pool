pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap(),
        Security::Message => server.expect("ERROR: program stops"),
        Security::Warning => server.unwrap_or("WARNING: check the server"),
        Security::NotFound => server.map_err(|a| "Not found: ".to_string() + a).unwrap(),
        Security::UnexpectedUrl => server.unwrap_err(),
    }
    .to_string()
}
