use super::{verify_password_hash, User, Post};
use crate::app::DbPool;

use crate::errors;
use crate::schema::{user::NewUser,form::PostsUpdateData};
use diesel::prelude::*;
use tracing::instrument;

#[instrument(name = "Add new user", skip(connection))]
pub fn db_add_user(user: User, connection: &DbPool) -> Result<User, errors::Error> {
    use super::schema::users::dsl::{username, users};
    let mut conn = connection.get().map_err(|e| {
        tracing::error!("Failed to get db connection pool!");

        errors::Error::new(
            Some(e.to_string()),
            Some("Failed to get db connection pool!".to_string()),
            errors::ErrorTypes::DbError,
        )
    })?;

    let result = users
        .filter(username.eq(&user.username))
        .select(User::as_select())
        .first(&mut conn);

    if result.is_ok() {
        tracing::error!("The username already exists!");
        return Err(errors::Error::new(
            None,
            Some("The username already exists!".to_string()),
            errors::ErrorTypes::Auth(errors::Auth::Authentication),
        ));
    }

    diesel::insert_into(users)
        .values(&user)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to add user: {:?} to the database!", user);
            errors::Error::new(Some(e.to_string()), None, errors::ErrorTypes::DbError)
        })?;
    tracing::info!("User: {:?} added successfully!", user);

    Ok(user)
}

#[instrument(name = "Check logged in data", skip(connection))]
pub fn db_check_user(user: NewUser, connection: &DbPool) -> Result<User, errors::Error> {
    use super::schema::users::dsl::{username, users};
    let mut conn = connection.get().map_err(|e| {
        tracing::error!("Failed to get db connection pool!");
        errors::Error::new(
            Some(e.to_string()),
            Some("Failed to get db connection pool!".to_string()),
            errors::ErrorTypes::DbError,
        )
    })?;

    let founded_user = users
        .filter(username.eq(&user.username.0))
        .select(User::as_select())
        .first(&mut conn)
        .map_err(|e| {
            tracing::info!("User with username {} not found!", &user.username.0);
            errors::Error::new(
                Some(e.to_string()),
                Some("User not found. Try to check your username!".to_string()),
                errors::ErrorTypes::Auth(errors::Auth::Authentication),
            )
        })?;

    verify_password_hash(&founded_user.password, &user.password.0)?;
    tracing::info!("User: {:?} verified!", founded_user);

    
    Ok(founded_user)
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
    tracing::info!("Founded user: {:?} successfully!", user);

    Ok(user)
}



#[instrument(name = "Get all posts", skip(connection))]
pub fn db_get_posts(connection: &DbPool) -> Result<Vec<Post>, errors::Error> {
    use super::schema::posts::{dsl::*,important};
    let mut conn = connection.get().map_err(|e| {
        tracing::error!("Failed to get db connection pool!");

        errors::Error::new(
            Some(e.to_string()),
            Some("Failed to get db connection pool!".to_string()),
            errors::ErrorTypes::DbError,
        )
    })?;

    let post = posts
        .select(Post::as_returning())
        .order(important.desc())
        .load(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to get posts");
            errors::Error::new(
                Some(e.to_string()),
                Some("Can not get items from table Posts!".to_string()),
                errors::ErrorTypes::DbError,
            )
        })?;
    tracing::info!("Got posts:{:?} from db!", post);
    

    Ok(post)
}

#[instrument(name = "Add new post", skip(connection))]
pub fn db_add_post(post: Post, connection: &DbPool) -> Result<Post, errors::Error> {
    use super::schema::posts::dsl::*;
    let mut conn = connection.get().map_err(|e| {
        tracing::error!("Failed to get db connection pool!");

        errors::Error::new(
            Some(e.to_string()),
            Some("Failed to get db connection pool!".to_string()),
            errors::ErrorTypes::DbError,
        )
    })?;


    diesel::insert_into(posts)
        .values(&post)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to add new post: {:?} to the database!", post);
            errors::Error::new(Some(e.to_string()), None, errors::ErrorTypes::DbError)
        })?;
    tracing::info!("Post: {:?} added successfully!", post);

    Ok(post)
}

#[instrument(name = "Update posts's impotant field!", skip(connection))]
pub fn db_update_post(data:PostsUpdateData, connection: &DbPool) -> Result<Post, errors::Error> {
    use super::schema::posts::dsl::*;
    let mut conn = connection.get().map_err(|e| {
        tracing::error!("Failed to get db connection pool!");

        errors::Error::new(
            Some(e.to_string()),
            Some("Failed to get db connection pool!".to_string()),
            errors::ErrorTypes::DbError,
        )
    })?;

    let post = diesel::update(posts.find(data.id))
        .set(important.eq(data.important))
        .returning(Post::as_returning())
        .get_result(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to update the post");
            errors::Error::new(
                Some(e.to_string()),
                Some("Can not update this post!".to_string()),
                errors::ErrorTypes::DbError,
            )
        })?;
    tracing::info!("Updated post:{:?} from db!", post);
    

    Ok(post)
}