use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FormData {
    pub username: String,
    pub password: String,
}
