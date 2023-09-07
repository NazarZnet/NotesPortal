use std::collections::HashSet;

use super::{verify_password_hash, Post, User};
use crate::app::DbPool;

use crate::errors;
use crate::schema::user::NewUser;
use common::{PostsUpdateForm, ResponsePost};
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

/// The function `db_get_posts` retrieves all posts from the database, along with information about
/// whether each post is marked as important for a specific user.
///
/// Arguments:
///
/// * `user_id`: The `user_id` parameter is of type `uuid::Uuid` and represents the ID of the user for
/// whom we want to retrieve the posts.
/// * `connection`: The `connection` parameter is of type `&DbPool`, which is a reference to a database
/// connection pool. It is used to establish a connection to the database and execute queries.
///
/// Returns:
///
/// The function `db_get_posts` returns a `Result` containing a `Vec` of `ResponsePost` structs or an
/// `errors::Error` if there was an error retrieving the posts from the database.

#[instrument(name = "Get all posts", skip(connection))]
pub fn db_get_posts(
    user_id: uuid::Uuid,
    connection: &DbPool,
) -> Result<Vec<ResponsePost>, errors::Error> {
    use super::schema::important_posts;
    use super::schema::posts;

    let mut conn = connection.get().map_err(|e| {
        tracing::error!("Failed to get db connection pool!");

        errors::Error::new(
            Some(e.to_string()),
            Some("Failed to get db connection pool!".to_string()),
            errors::ErrorTypes::DbError,
        )
    })?;

    //get all available posts
    let all_posts: Vec<Post> = posts::table
        .select(Post::as_returning())
        .order(posts::created_at.desc())
        .load(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to get all posts");
            errors::Error::new(
                Some(e.to_string()),
                Some("Can not get all items from table posts!".to_string()),
                errors::ErrorTypes::DbError,
            )
        })?;

    //get important posts for current user
    let important_posts: HashSet<uuid::Uuid> = important_posts::table
        .select(important_posts::post_id)
        .filter(important_posts::user_id.eq(user_id))
        .load(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to get important posts");
            errors::Error::new(
                Some(e.to_string()),
                Some("Can not get important items from table important_posts!".to_string()),
                errors::ErrorTypes::DbError,
            )
        })?
        .into_iter()
        .collect();

    //group important and not posts
    let mut response_posts: Vec<ResponsePost> = all_posts
        .into_iter()
        .map(|post| {
            let important = important_posts.contains(&post.id);
            ResponsePost {
                id: post.id,
                title: post.title,
                description: post.description,
                important,
                created_at: post.created_at,
            }
        })
        .collect();

    //sort the posts so that important posts come first
    response_posts.sort_by(|a, b| b.important.cmp(&a.important));

    tracing::info!("Got posts:{:?} from db!", response_posts);

    Ok(response_posts)
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
pub fn db_add_post(post: Post, connection: &DbPool) -> Result<ResponsePost, errors::Error> {
    use super::schema::posts::dsl::*;
    let mut conn = connection.get().map_err(|e| {
        tracing::error!("Failed to get db connection pool!");

        errors::Error::new(
            Some(e.to_string()),
            Some("Failed to get db connection pool!".to_string()),
            errors::ErrorTypes::DbError,
        )
    })?;

    let db_post: Post = diesel::insert_into(posts)
        .values(&post)
        .get_result(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to add new post: {:?} to the database!", post);
            errors::Error::new(Some(e.to_string()), None, errors::ErrorTypes::DbError)
        })?;

    let post = ResponsePost {
        id: db_post.id,
        important: false,
        title: db_post.title,
        description: db_post.description,
        created_at: db_post.created_at,
    };
    tracing::info!("Post: {:?} added successfully!", post);

    Ok(post)
}

/// The function `db_update_post` updates the important field of a post in the database based on the
/// provided user ID and post data.
///
/// Arguments:
///
/// * `user_id`: The `user_id` parameter is of type `uuid::Uuid`, which represents a universally unique
/// identifier (UUID) for a user.
/// * `data`: The `data` parameter in the `db_update_post` function is of type `PostsUpdateForm`. It
/// represents the data needed to update a post.
/// * `connection`: The `connection` parameter is of type `&DbPool`, which is a reference to a database
/// connection pool. It is used to establish a connection to the database and perform database
/// operations.
///
/// Returns:
///
/// The function `db_update_post` returns a `Result<(), errors::Error>`.
#[instrument(name = "Update posts's impotant field!", skip(connection))]
pub fn db_update_post(
    user_id: uuid::Uuid,
    data: PostsUpdateForm,
    connection: &DbPool,
) -> Result<(), errors::Error> {
    use super::schema::important_posts;
    let mut conn = connection.get().map_err(|e| {
        tracing::error!("Failed to get db connection pool!");

        errors::Error::new(
            Some(e.to_string()),
            Some("Failed to get db connection pool!".to_string()),
            errors::ErrorTypes::DbError,
        )
    })?;

    if data.important {
        // Insert into important_posts if the post should be marked as important
        diesel::insert_into(important_posts::table)
            .values((
                important_posts::user_id.eq(user_id),
                important_posts::post_id.eq(data.id),
            ))
            .execute(&mut conn)
            .map_err(|e| {
                tracing::error!("Failed to make current post important!");
                errors::Error::new(Some(e.to_string()), None, errors::ErrorTypes::DbError)
            })?;
    } else {
        // Delete from important_posts if the post should not be marked as important
        diesel::delete(
            important_posts::table
                .filter(important_posts::user_id.eq(user_id))
                .filter(important_posts::post_id.eq(data.id)),
        )
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to make current post not important!");
            errors::Error::new(Some(e.to_string()), None, errors::ErrorTypes::DbError)
        })?;
    }
    tracing::info!("Updated post with id:{:?}", data.id);

    Ok(())
}
