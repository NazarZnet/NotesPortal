pub mod models;
pub mod schema;

pub use models::*;

use crate::errors;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

//hashing password with argon2 and random salt
pub fn hash_password(password: String) -> Result<String, errors::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| errors::Error {
            cause: Some(e.to_string()),
            message: Some("Password hashing error".into()),
            error_type: errors::ErrorTypes::DbError,
        })?
        .to_string();

    Ok(hashed_password)
}
