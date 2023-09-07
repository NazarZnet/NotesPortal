use crate::{db::Post, errors};

use time::OffsetDateTime;

#[derive(Debug)]
pub struct Title(pub String);

impl Title {
    /// The `parse` function takes a `title` string as input and returns a `Result` containing a `Title`
    /// object if the title is valid, or an `Error` object if the title is empty.
    ///
    /// Arguments:
    ///
    /// * `title`: The `title` parameter is a string reference (`&str`) representing the title of a
    /// post.
    ///
    /// Returns:
    ///
    /// The function `parse` returns a `Result` type. If the `title` is not empty, it returns
    /// `Ok(Title(title.to_owned()))`, where `Title` is a struct that takes ownership of the `title`
    /// string. If the `title` is empty, it returns `Err(errors::Error::new(None,Some("Ivalid post's
    /// title! It can not be
    fn parse(title: &str) -> Result<Title, errors::Error> {
        if title.trim().is_empty() {
            return Err(errors::Error::new(
                None,
                Some("Ivalid post's title! It can not be empty!".to_string()),
                errors::ErrorTypes::ValidationError,
            ));
        }

        Ok(Title(title.to_owned()))
    }
}

#[derive(Debug)]
/// The `NewPost` struct represents a new post with a title, optional description, and user ID.
///
/// Properties:
///
/// * `title`: The `title` property is of type `Title`. It is a required field for creating a new post.
/// * `description`: The `description` property is an optional field that can contain a `String` value.
/// It is wrapped in an `Option` type, which means it can either be `Some(value)` if a description is
/// provided, or `None` if no description is given.
/// * `user_id`: The `user_id` property is of type `uuid::Uuid`, which represents a universally unique
/// identifier. It is used to uniquely identify a user.
pub struct NewPost {
    pub title: Title,
    pub description: Option<String>,
    pub user_id: uuid::Uuid,
}

impl NewPost {
    /// The function `parse` takes a title, description, and user ID as input and returns a `NewPost`
    /// struct if successful.
    ///
    /// Arguments:
    ///
    /// * `title`: A string representing the title of the post.
    /// * `description`: The `description` parameter is an optional string. It is wrapped in an `Option`
    /// type, which means it can either be `Some(string)` or `None`.
    /// * `user_id`: The `user_id` parameter is of type `uuid::Uuid`, which represents a universally unique
    /// identifier (UUID). It is used to uniquely identify a user.
    ///
    /// Returns:
    ///
    /// a `Result` type with the variant `Ok` containing a `NewPost` struct, or an `errors::Error` if there
    /// was an error during parsing.
    pub fn parse(
        title: &str,
        description: &Option<String>,
        user_id: uuid::Uuid,
    ) -> Result<NewPost, errors::Error> {
        let title = Title::parse(title)?;
        Ok(NewPost {
            title,
            description: description.to_owned(),
            user_id: user_id.to_owned(),
        })
    }
    /// The function builds a DB Post object by converting data from the current object.
    ///
    /// Returns:
    ///
    /// The function `build` returns a `Post` object.
    pub fn build(&self) -> Post {
        tracing::info!("Converting data to DB Post!");
        Post {
            id: uuid::Uuid::new_v4(),
            user_id: self.user_id,
            title: self.title.0.clone(),
            description: self.description.clone(),
            created_at: OffsetDateTime::now_utc(),
        }
    }
}
