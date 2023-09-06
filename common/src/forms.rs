use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// The `UserFormData` struct represents user input data including a username and password.
///
/// Properties:
///
/// * `username`: A string that represents the username entered by the user. This is typically used for
/// identification purposes.
/// * `password`: The `password` property is a string that represents the user's password.
pub struct UserFormData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// The `PostsFormData` struct represents data for creating a post, including a title and an optional
/// description.
///
/// Properties:
///
/// * `title`: A string that represents the title of a post.
/// * `description`: The `description` property is an optional field that can contain a string value. It
/// is marked as `Option<String>`, which means it can either be `Some(value)` where `value` is a string,
/// or `None` if no value is provided.
pub struct PostsFormData {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// The `PostsUpdateForm` struct represents a form used to update posts, containing an ID and a flag
/// indicating importance.
///
/// Properties:
///
/// * `id`: The `id` property is of type `uuid::Uuid`, which represents a universally unique identifier.
/// It is used to uniquely identify a post.
/// * `important`: The `important` property is a boolean value that indicates whether the post is
/// important or not.
pub struct PostsUpdateForm {
    pub id: uuid::Uuid,
    pub important: bool,
}
