use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Auth {
    Authentication,
    Authorization,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub enum ErrorTypes {
    ValidationError,
    DbError,
    Auth(Auth),
    JwtError,
    #[default]
    RequestError,
    DeserializeError,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
/// The `ErrorResponse` struct represents an error response with optional cause, message, and error type
/// in Rust.
///
/// Properties:
///
/// * `cause`: An optional string that represents the cause of the error. It provides additional
/// information about why the error occurred.
/// * `message`: The `message` property is an optional string that represents the error message. It
/// provides additional information about the error that occurred.
/// * `error_type`: ErrorTypes is an enum that represents the type of error that occurred. It could have
/// different variants such as BadRequest, NotFound, InternalServerError, etc.
pub struct ErrorResponse {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: ErrorTypes,
}

impl ErrorResponse {
    pub fn new(cause: Option<String>, message: Option<String>, error_type: ErrorTypes) -> Self {
        ErrorResponse {
            cause,
            message,
            error_type,
        }
    }
}
