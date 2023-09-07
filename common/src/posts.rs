use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// The `ResponsePost` struct represents a post with an ID, user ID, importance flag, title,
/// description, and creation timestamp.
///
/// Properties:
///
/// * `id`: The `id` property is of type `Uuid` and represents the unique identifier of the response
/// post.
/// * `important`: A boolean value indicating whether the post is important or not.
/// * `title`: The `title` property is a string that represents the title of the post.
/// * `description`: The `description` property is an optional field that can contain a string value. It
/// is marked as `Option<String>`, which means it can either be `Some(value)` where `value` is a string,
/// or `None` if no value is provided.
/// * `created_at`: The `created_at` property is of type `OffsetDateTime` and is used to store the
/// timestamp when the post was created. It is annotated with `#[serde(with="time::serde::rfc3339")]`
/// which indicates that it should be serialized and deserialized using the `time::
pub struct ResponsePost {
    pub id: Uuid,
    pub important: bool,
    pub title: String,
    pub description: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
