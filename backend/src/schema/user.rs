use crate::db::{hash_password, User};

use lazy_static::lazy_static;
use regex::Regex;
use time::OffsetDateTime;

use crate::errors;

//regexes for name and password validations
lazy_static! {
    //username should conatines only latters and digits and minimal length is 5
    static ref USERNAME_REGEX: Regex =
        Regex::new(r"^[a-zA-Z0-9]{5,}$").expect("Ivalid regular expression");
    //password must have minimum one number and minimum 8 symbols
    static ref PASSWORD_REGEX: Regex =
        Regex::new(r"^[a-zA-Z\d]+$").expect("Ivalid regular expression");
}
#[derive(Debug)]
pub struct Username(pub String);

impl Username {
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
    fn parse(password: &str) -> Result<Password, errors::Error> {
        match PASSWORD_REGEX.is_match(password){
            true=>Ok(Password(password.to_string())),
            false=>Err(errors::Error::new(None,Some("Invalid password! Must contain at least 8 characters, one or more of which are numbers, the rest are letters!".to_string()),errors::ErrorTypes::ValidationError))
        }
    }
}
#[derive(Debug)]
pub struct NewUser {
    pub username: Username,
    pub password: Password,
}

impl NewUser {
    pub fn parse<T: AsRef<str>>(username: T, password: T) -> Result<NewUser, errors::Error> {
        tracing::info!("Starting data validation!");
        let username = Username::parse(username.as_ref())?;
        let password = Password::parse(password.as_ref())?;
        tracing::info!("Successfully validated!");
        Ok(NewUser { username, password })
    }
    pub fn to_user(&self) -> Result<User, errors::Error> {
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
