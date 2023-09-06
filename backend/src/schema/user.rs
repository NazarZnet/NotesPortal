use crate::db::{hash_password, User};

use lazy_static::lazy_static;
use onig::Regex;
use time::OffsetDateTime;

use crate::errors;

//regexes for name and password validations
lazy_static! {
    //username should conatines only latters and digits and minimal length is 5
    static ref USERNAME_REGEX: Regex =
        Regex::new(r"^[a-zA-Z0-9]{5,}$").expect("Ivalid regular expression");
    //password must have minimum one number and minimum 8 symbols
    static ref PASSWORD_REGEX: Regex =
        Regex::new(r#"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$"#).expect("Ivalid regular expression");
}

#[derive(Debug)]
pub struct Username(pub String);

impl Username {
    /// The `parse` function takes a username as input and returns a `Result` containing a `Username` if
    /// the username is valid, or an `Error` if the username is invalid.
    ///
    /// Arguments:
    ///
    /// * `username`: The `username` parameter is a string that represents the username that needs to be
    /// parsed.
    ///
    /// Returns:
    ///
    /// The `parse` function returns a `Result` type. If the `USERNAME_REGEX` matches the `username`
    /// string, it returns an `Ok` variant containing a `Username` struct with the `username` string as
    /// its value. If the `USERNAME_REGEX` does not match the `username` string, it returns an `Err`
    /// variant containing an `errors::Error` struct with an
    fn parse(username: &str) -> Result<Username, errors::Error> {
        match USERNAME_REGEX.is_match(username){
            true=>Ok(Username(username.to_string())),
            false=>Err(errors::Error::new(None,Some("Ivalid username! Must containes minimum 5 symbols and only letters and numbers!".to_string()),errors::ErrorTypes::ValidationError))
        }
    }
}
#[derive(Debug)]
pub struct Password(pub String);

impl Password {
    /// The `parse` function takes a password as input and returns a `Result` indicating whether the
    /// password is valid or not.
    ///
    /// Arguments:
    ///
    /// * `password`: The `password` parameter is a string that represents the password that needs to be
    /// parsed.
    ///
    /// Returns:
    ///
    /// The function `parse` returns a `Result` type. If the `PASSWORD_REGEX` matches the given
    /// `password`, it returns `Ok(Password(password.to_string()))`, where `Password` is a struct that
    /// wraps the password string. If the `PASSWORD_REGEX` does not match the given `password`, it
    /// returns `Err(errors::Error::new(None,Some("Invalid password! Must contain
    fn parse(password: &str) -> Result<Password, errors::Error> {
        match PASSWORD_REGEX.is_match(password){
            true=>Ok(Password(password.to_string())),
            false=>Err(errors::Error::new(None,Some("Invalid password! Must contain at least 8 characters, one or more of which are numbers, the rest are letters!".to_string()),errors::ErrorTypes::ValidationError))
        }
    }
}
#[derive(Debug)]
/// The `NewUser` struct represents a new user with a username and password.
///
/// Properties:
///
/// * `username`: The `username` property is of type `Username`. It represents the username of a new
/// user.
/// * `password`: The `password` property is of type `Password`.
pub struct NewUser {
    pub username: Username,
    pub password: Password,
}

impl NewUser {
    /// The `parse` function takes a username and password as input, validates them, and returns a
    /// `NewUser` struct if successful.
    ///
    /// Arguments:
    ///
    /// * `username`: The `username` parameter is the username provided by the user. It is of type `T`,
    /// which is a generic type that implements the `AsRef<str>` trait. This means that it can accept
    /// any type that can be converted to a string reference, such as `String`, `&
    /// * `password`: The `password` parameter is of type `T`, which is a generic type that implements
    /// the `AsRef<str>` trait. This means that it can accept any type that can be converted into a
    /// string reference.
    ///
    /// Returns:
    ///
    /// The function `parse` returns a `Result` type with the success case containing a `NewUser` struct
    /// and the error case containing an `errors::Error` type.
    pub fn parse<T: AsRef<str>>(username: T, password: T) -> Result<NewUser, errors::Error> {
        tracing::info!("Starting data validation!");
        let username = Username::parse(username.as_ref())?;
        let password = Password::parse(password.as_ref())?;
        tracing::info!("Successfully validated!");
        Ok(NewUser { username, password })
    }
    /// The function builds a DB User by converting data and hashing the password.
    ///
    /// Returns:
    ///
    /// a `Result` type with the success variant containing a `User` object and the error variant
    /// containing an `errors::Error` object.
    pub fn build(&self) -> Result<User, errors::Error> {
        tracing::info!("Converting data to DB User!");
        let hashed_password = hash_password(&self.password.0)?;
        Ok(User {
            id: uuid::Uuid::new_v4(),
            username: self.username.0.to_string(),
            password: hashed_password,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        })
    }
}
