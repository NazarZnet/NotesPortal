use std::default;

use serde::{Serialize,Deserialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct ResponseUser {
    pub id: Uuid,
    pub username: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct LoginResponse{
    pub status: String,
    pub access:String,
    pub refresh:String
}

#[derive(Debug, Serialize,Deserialize,PartialEq)]
pub enum Auth {
    Authentication,
    Authorization,
}

#[derive(Debug, Serialize,Deserialize,Default,PartialEq)]
pub enum ErrorTypes {
    ValidationError,
    DbError,
    Auth(Auth),
    JwtError,
    #[default]
    RequestError,
}

#[derive(Debug,Serialize,Deserialize,Default)]
pub struct ErrorResponse{
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: ErrorTypes
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