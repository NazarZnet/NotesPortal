use diesel::prelude::*;
use time::OffsetDateTime;
use uuid::Uuid;

use super::schema::{posts, users};
use serde::{Deserialize, Serialize};

/// The DB User model.
///
/// Properties:
///
/// * `id`: The `id` property is of type `Uuid` and represents the unique identifier for a user.
/// * `username`: The `username` property is of type `String` and represents the username of a user. It
/// is used to uniquely identify a user in the system.
/// * `password`: The `password` property is a `String` field that stores the user's password. It is
/// marked with `#[serde(skip_serializing)]`, which means that it will not be included when serializing
/// the `User` struct into JSON or any other serialized format. This is typically done for security
/// * `created_at`: The `created_at` property is of type `OffsetDateTime`. It represents the date and
/// time when the user was created. The `OffsetDateTime` type is used to store a date and time along
/// with the offset from UTC (Coordinated Universal Time).
/// * `updated_at`: The `updated_at` property is a field of type `OffsetDateTime` that represents the
/// date and time when the user was last updated. It is used to keep track of when the user's
/// information was last modified.
#[derive(Queryable, Selectable, Serialize, Insertable, Debug)]
#[diesel(table_name =users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// The DB Post model.
///
/// Properties:
///
/// * `id`: The `id` property is of type `Uuid` and represents the unique identifier of the post.
/// * `user_id`: The `user_id` property is of type `Uuid` and represents the unique identifier of the
/// user who created the post.
/// * `title`: The `title` property is a string that represents the title of the post.
/// * `description`: The `description` property is an optional field that can contain a string value. It
/// is marked as `Option<String>`, which means it can either be `Some(value)` where `value` is a string,
/// or `None` if no value is provided.
/// * `created_at`: The `created_at` property is of type `OffsetDateTime` and is annotated with
/// `#[serde(with="time::serde::rfc3339")]`. This annotation specifies that the `created_at` property
/// should be serialized and deserialized using the `rfc3339` format provided by the
#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
