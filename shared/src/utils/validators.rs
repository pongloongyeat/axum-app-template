use std::sync::LazyLock;

use email_address::EmailAddress;
use fancy_regex::Regex;

const PASSWORD_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,12}$").unwrap()
});
const INVALID_PASSWORD_MESSAGE: &'static str = "Password must contain a minimum of 8-12 characters, with at least 1 uppercase letter, 1 lowercase letter, 1 number and 1 special character.";

pub fn is_email_valid(email: &str) -> Option<Vec<String>> {
    if !EmailAddress::is_valid(email) {
        Some(vec![format!("{email} is not a valid email address.")])
    } else {
        None
    }
}

pub fn is_password_valid(password: &str) -> Option<Vec<String>> {
    match PASSWORD_REGEX.is_match(password) {
        Ok(is_match) => {
            if is_match {
                None
            } else {
                Some(vec![INVALID_PASSWORD_MESSAGE.to_string()])
            }
        }
        Err(err) => {
            tracing::error!("{err}");
            panic!("{err}");
        }
    }
}
