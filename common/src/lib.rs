pub mod errors;
pub mod forms;
pub mod posts;
pub mod user;

pub use errors::*;
pub use forms::*;
pub use posts::*;
pub use user::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ApiResponse {
    pub status: String,
}
