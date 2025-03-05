use std::sync::LazyLock;

use email_address::EmailAddress;
use fancy_regex::Regex;

const PASSWORD_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$").unwrap()
});
const INVALID_PASSWORD_MESSAGE: &'static str = "Password must contain a minimum of 8 characters, with at least 1 uppercase letter, 1 lowercase letter, 1 number and 1 special character.";

pub fn is_email_valid(email: &str) -> Result<(), serde_valid::validation::Error> {
    EmailAddress::is_valid(email)
        .then(|| ())
        .ok_or(serde_valid::validation::Error::Custom(format!(
            "{email} is not a valid email address."
        )))
}

pub fn is_password_valid(password: &str) -> Result<(), serde_valid::validation::Error> {
    if PASSWORD_REGEX.is_match(password).is_ok() {
        Ok(())
    } else {
        Err(serde_valid::validation::Error::Custom(
            INVALID_PASSWORD_MESSAGE.to_string(),
        ))
    }
}
