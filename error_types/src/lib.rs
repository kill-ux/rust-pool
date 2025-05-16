pub type Utc = chrono::Utc;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError<'a> {
    pub form_values: (&'a str, String),
    pub date: String,
    pub err: &'a str,
}

impl<'a> FormError<'a> {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

trait IsSymbol {
    fn is_symbol(&self) -> bool;
}

impl IsSymbol for char {
    fn is_symbol(&self) -> bool {
        matches!(*self, '!'..='/' | ':'..='@' | '['..='`' | '{'..='~')
    }
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        let mut alpha = false;
        let mut numeric = false;
        let mut symbol = false;
        for ch in self.password.chars() {
            if ch.is_alphabetic() {
                alpha = true;
            } else if ch.is_numeric() {
                numeric = true;
            } else if ch.is_symbol() {
                symbol = true;
            }

            if alpha && numeric && symbol && self.password.len() > 8 && !self.name.is_empty() {
                return Ok(());
            }
        }
        if self.name.is_empty() {
            return Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty",
            ));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }
        if !numeric || !symbol {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }

        Err(FormError::new(
            "password",
            self.password.clone(),
            "Password should be at least 8 characters long",
        ))
    }
}
