use super::{verify_password_hash, User};
use crate::{app::DbPool, schema::user::NewUser};

use crate::errors;
use diesel::prelude::*;
use tracing::instrument;

#[instrument(name = "Add new user", skip(connection))]
pub fn db_add_user(data: NewUser, connection: &DbPool) -> Result<User, errors::Error> {
    use super::schema::users::dsl::{username, users};
    let mut conn = connection.get().map_err(|e| {
        tracing::error!("Failed to get db connection pool!");

        errors::Error::new(
            Some(e.to_string()),
            Some("Failed to get db connection pool!".to_string()),
            errors::ErrorTypes::DbError,
        )
    })?;
    let user = data.to_user()?;

    let result = users
        .filter(username.eq(&user.username))
        .select(User::as_select())
        .first(&mut conn);

    if result.is_ok() {
        tracing::error!("The username already exists!");
        return Err(errors::Error::new(
            None,
            Some("The username already exists!".to_string()),
            errors::ErrorTypes::ValidationError,
        ));
    }

    diesel::insert_into(users)
        .values(&user)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to add user: {:?} to the database!", user);
            errors::Error::new(Some(e.to_string()), None, errors::ErrorTypes::DbError)
        })?;

    Ok(user)
}

#[instrument(name = "Check logged in data", skip(connection))]
pub fn db_check_user(data: NewUser, connection: &DbPool) -> Result<User, errors::Error> {
    use super::schema::users::dsl::{username, users};
    let mut conn = connection.get().map_err(|e| {
        tracing::error!("Failed to get db connection pool!");
        errors::Error::new(
            Some(e.to_string()),
            Some("Failed to get db connection pool!".to_string()),
            errors::ErrorTypes::DbError,
        )
    })?;

    let user = users
        .filter(username.eq(&data.username.0))
        .select(User::as_select())
        .first(&mut conn)
        .map_err(|e| {
            tracing::info!("User with username {} not found!", &data.username.0);
            errors::Error::new(
                Some(e.to_string()),
                Some("User not found. Try to check your username!".to_string()),
                errors::ErrorTypes::ValidationError,
            )
        })?;

    verify_password_hash(&user.password, &data.password.0)?;

    Ok(user)
}

#[instrument(name = "Find the user in db", skip(connection))]
pub fn db_find_user(user_id: uuid::Uuid, connection: &DbPool) -> Result<User, errors::Error> {
    use super::schema::users::dsl::{id, users};
    let mut conn = connection.get().map_err(|e| {
        tracing::error!("Failed to get db connection pool!");

        errors::Error::new(
            Some(e.to_string()),
            Some("Failed to get db connection pool!".to_string()),
            errors::ErrorTypes::DbError,
        )
    })?;

    let user = users
        .filter(id.eq(&user_id))
        .select(User::as_returning())
        .first(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to find user with id: {}", user_id);
            errors::Error::new(
                Some(e.to_string()),
                Some("Invalid Uuid inside refresh token!".to_string()),
                errors::ErrorTypes::Auth(errors::Auth::Authentication),
            )
        })?;

    Ok(user)
}
