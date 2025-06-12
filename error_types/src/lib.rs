use chrono::Utc;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError<'a> {
    pub form_values: (&'a str, String),
    pub date: String,
    pub err: &'a str,
}

impl<'a> FormError<'a> {
    pub fn new(field_name: &'a str, field_value: String, err: &'a str) -> Self {
        FormError {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}
// Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    // expected public fields
    pub name : String,
    pub password : String
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name == "" {
            return Err(FormError::new("name",  self.name.clone(), "Username is empty"))
        }
        if self.password.len() < 8 {
            return Err(FormError::new("password",  self.password.clone(), "Password should be at least 8 characters long"))
        }
        let mut is_char = false;
        let mut is_num = false;
        let mut is_special = false;
        for c in self.password.chars() {
            if c.is_alphabetic() {
                is_char = true;
            }else if c.is_numeric() {
                is_num = true;
            }else if !c.is_alphanumeric() {
                is_special = true;
            }
        }
        if !is_char || !is_num || !is_special {
            return Err(FormError::new("password",  self.password.clone(), "Password should be a combination of ASCII numbers, letters and symbols"))
        }
        Ok(())
    }
}