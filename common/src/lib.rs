pub mod errors;


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

#[derive(Debug,Serialize,Deserialize)]
pub struct ErrorResponse{
    pub cause: Option<String>,
    pub message: Option<String>
}