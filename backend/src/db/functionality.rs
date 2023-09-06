use super::{verify_password_hash, Post, User};
use crate::app::DbPool;

use crate::errors;
use crate::schema::user::NewUser;
use common::PostsUpdateForm;
use diesel::prelude::*;
use tracing::instrument;
/// The function `db_add_user` adds a new user to a database, checking if the username already exists
/// before inserting the user.
///
/// Arguments:
///
/// * `user`: The `user` parameter is of type `User`, which represents the user data that you want to
/// add to the database.
/// * `connection`: The `connection` parameter is of type `&DbPool`, which is a reference to a database
/// connection pool. It is used to establish a connection to the database and perform database
/// operations.
///
/// Returns:
///
/// The function `db_add_user` returns a `Result<User, errors::Error>`.

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
/// The function `db_check_user` checks if a user is logged in by querying the database and verifying
/// their password.
///
/// Arguments:
///
/// * `user`: The `user` parameter is of type `NewUser`, which represents the data of a new user that
/// needs to be checked in the database. It contains the username and password of the user.
/// * `connection`: The `connection` parameter is of type `DbPool`, which is a connection pool to the
/// database. It is used to establish a connection to the database and perform database operations.
///
/// Returns:
///
/// The function `db_check_user` returns a `Result<User, errors::Error>`.

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

/// The function `db_find_user` is used to find a user in the database based on their user ID.
///
/// Arguments:
///
/// * `user_id`: The `user_id` parameter is of type `uuid::Uuid` and represents the unique identifier
/// of the user you want to find in the database.
/// * `connection`: The `connection` parameter is of type `&DbPool`, which is a reference to a database
/// connection pool. It is used to establish a connection to the database and perform database
/// operations.
///
/// Returns:
///
/// The function `db_find_user` returns a `Result<User, errors::Error>`.
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

/// The function `db_get_posts` retrieves all posts from a database table and returns them as a vector,
/// with an optional error handling mechanism.
///
/// Arguments:
///
/// * `connection`: The `connection` parameter is of type `DbPool`, which represents a connection pool
/// to the database. It is used to establish a connection to the database and execute queries.
///
/// Returns:
///
/// The function `db_get_posts` returns a `Result` containing a `Vec<Post>` or an `errors::Error`.

#[instrument(name = "Get all posts", skip(connection))]
pub fn db_get_posts(connection: &DbPool) -> Result<Vec<Post>, errors::Error> {
    use super::schema::posts::{dsl::*, important};
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
/// The function `db_add_post` adds a new post to the database using a connection pool.
///
/// Arguments:
///
/// * `post`: The `post` parameter is of type `Post`, which represents the data for the new post that
/// you want to add to the database.
/// * `connection`: The `connection` parameter is of type `&DbPool`, which is a reference to a database
/// connection pool. It is used to establish a connection to the database and execute the database
/// operations.
///
/// Returns:
///
/// The function `db_add_post` returns a `Result<Post, errors::Error>`.

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
/// This function updates the "important" field of a post in a database.
///
/// Arguments:
///
/// * `data`: PostsUpdateForm - a struct containing the data to update the post's important field. It
/// likely has a field called "id" to identify the post and a field called "important" to update the
/// value.
/// * `connection`: The `connection` parameter is of type `&DbPool`, which is a reference to a database
/// connection pool. It is used to establish a connection to the database and perform the update
/// operation on the `posts` table.
///
/// Returns:
///
/// The function `db_update_post` returns a `Result<Post, errors::Error>`.

#[instrument(name = "Update posts's impotant field!", skip(connection))]
pub fn db_update_post(data: PostsUpdateForm, connection: &DbPool) -> Result<Post, errors::Error> {
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
