use diesel::prelude::*;
use time::OffsetDateTime;
use uuid::Uuid;

use super::schema::{posts, users};
use serde::{Serialize, Deserialize};

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

#[derive(Queryable, Selectable,Insertable,Serialize,Deserialize,Debug)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub important:bool,
    pub title: String,
    pub description: Option<String>,
    #[serde(with="time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
