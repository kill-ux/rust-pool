pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    let s = match security_level {
        Security::Unknown => server.unwrap(),
        Security::Message => server.expect("ERROR: program stops"),
        Security::Warning => server.unwrap_or("WARNING: check the server"),
        Security::NotFound => match server {
            Ok(a) => a,
            Err(msg_err) => &format!("Not found: {msg_err}")
         },
        Security::UnexpectedUrl => server.unwrap_err(),
    } ;

    s.to_string()
}
