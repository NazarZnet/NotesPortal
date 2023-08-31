pub mod functionality;
pub mod models;
pub mod schema;
use common::errors;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
pub use functionality::*;
pub use models::*;

//hashing password with argon2 and random salt
pub fn hash_password(password: &str) -> Result<String, errors::Error> {
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

//check hashed password from database and form password
pub fn verify_password_hash(hashed_password: &str, password: &str) -> Result<(), errors::Error> {
    let parsed_hash = PasswordHash::new(hashed_password).map_err(|e| errors::Error {
        cause: Some(e.to_string()),
        message: Some("Password hashing error".into()),
        error_type: errors::ErrorTypes::DbError,
    })?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|e| {
            tracing::error!("Failed to validate password!");
            errors::Error::new(
                Some(e.to_string()),
                Some("Incorrect password. Try again!".into()),
                errors::ErrorTypes::Auth(errors::Auth::Authentication),
            )
        })?;

    Ok(())
}
