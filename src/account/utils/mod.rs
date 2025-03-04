use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::{rngs::OsRng, seq::SliceRandom};

use crate::error::{AppError, AppResult};

pub mod extractors;

const OTP_ALLOWED_VALUES: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

pub fn hash_password(password: &str) -> AppResult<String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(AppError::from)
}

pub fn generate_otp(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut values = Vec::from(OTP_ALLOWED_VALUES);
    values.shuffle(&mut rng);

    values
        .iter()
        .take(length)
        .map(|val| val.to_string())
        .collect()
}
