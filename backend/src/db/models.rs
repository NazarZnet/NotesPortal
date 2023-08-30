use crate::errors;
use diesel::prelude::*;
use time::OffsetDateTime;
use uuid::Uuid;

use super::hash_password;
use super::schema::{posts, users};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name =users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl NewUser {
    pub fn new(username: impl ToString, password: impl ToString) -> Result<NewUser, errors::Error> {
        let password_hash = hash_password(password.to_string())?;
        Ok(NewUser {
            username: username.to_string(),
            password: password_hash,
        })
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
}
