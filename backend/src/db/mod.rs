pub mod functionality;
pub mod models;
pub mod schema;
use crate::errors;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
pub use functionality::*;
pub use models::*;

/// The function `hash_password` takes a password as input, generates a salt, and uses the Argon2
/// algorithm to hash the password, returning the hashed password as a string.
///
/// Arguments:
///
/// * `password`: The `password` parameter is a string that represents the user's password that needs to
/// be hashed.
///
/// Returns:
///
/// a `Result` type with the success variant containing a `String` representing the hashed password, and
/// the error variant containing an `errors::Error` object.
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

/// The function `verify_password_hash` verifies if a given password matches a hashed password using the
/// Argon2 algorithm.
///
/// Arguments:
///
/// * `hashed_password`: The `hashed_password` parameter is a string that represents the hashed version
/// of a password. It is the result of applying a cryptographic hash function to the original password.
/// * `password`: The `password` parameter is the plain text password that needs to be verified against
/// the hashed password.
///
/// Returns:
///
/// The function `verify_password_hash` returns a `Result<(), errors::Error>`.

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
