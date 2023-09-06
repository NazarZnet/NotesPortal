use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FormData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostsFormData {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct PostsUpdateData{
    pub id:uuid::Uuid,
    pub important:bool
}
