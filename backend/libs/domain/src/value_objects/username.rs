use regex::Regex;

use crate::errors::DomainError;

#[derive(Debug, Clone, Default, fake::Dummy)]
pub struct Username {
    pub value: String,
}

impl Username {
    pub fn new(value: String) -> Result<Self, DomainError> {
        let re = Regex::new(r"^[A-Za-z\d_]{5,32}$").unwrap();
        if re.is_match(value.as_str()) {
            Ok(Self { value })
        } else {
            Err(DomainError::ValidationError("Invalid Username".to_string()))
        }
    }
}

impl PartialEq for Username {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
