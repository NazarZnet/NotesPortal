use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// The `ResponseUser` struct represents a user with an ID, username, and timestamps for creation and
/// update.
///
/// Properties:
///
/// * `id`: The `id` property is of type `Uuid`, which is a universally unique identifier. It is used to
/// uniquely identify a user.
/// * `username`: The `username` property is a string that represents the username of a user. It is used
/// to uniquely identify a user in the system.
/// * `created_at`: The `created_at` property represents the date and time when the user was created. It
/// is of type `OffsetDateTime`, which is a struct that represents a date and time with an offset from
/// UTC. This allows for accurate representation of time across different time zones.
/// * `updated_at`: The `updated_at` property is a field that represents the date and time when the user
/// was last updated. It is of type `OffsetDateTime`, which is a struct that represents a date and time
/// with an offset from UTC.
pub struct ResponseUser {
    pub id: Uuid,
    pub username: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The `LoginResponse` struct represents the response received after a login request, containing the
/// status, access token, and refresh token.
///
/// Properties:
///
/// * `status`: A string that represents the status of the login response. It could be "success" or
/// "failure" or any other relevant status.
/// * `access`: The `access` property is a string that represents the access token for the logged-in
/// user. This token is typically used to authenticate and authorize the user to access protected
/// resources or perform certain actions within the system.
/// * `refresh`: The `refresh` property is a string that represents the refresh token. A refresh token
/// is a special token that is used to obtain a new access token when the current access token expires.
/// It is typically used in authentication systems to provide a seamless user experience without
/// requiring the user to re-enter their credentials.
pub struct LoginResponse {
    pub status: String,
    pub access: String,
    pub refresh: String,
}
