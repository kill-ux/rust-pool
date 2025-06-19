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
        Security::Unknown => server.unwrap(),
        Security::Message => server.expect("ERROR: program stops"),
        Security::Warning => server.unwrap_or("WARNING: check the server"),
        Security::NotFound => server.unwrap_or_else(|a: &str| {
            err = "Not found: ".to_string() + a;
            &err
        }),
        Security::UnexpectedUrl => server.unwrap_err(),
    }
    .to_string()
}
