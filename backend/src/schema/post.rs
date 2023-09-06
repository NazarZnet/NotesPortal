use crate::{db:: Post, errors};

use time::OffsetDateTime;

#[derive(Debug)]
pub struct Title(pub String);

impl Title {
    fn parse(title: &str) -> Result<Title, errors::Error> {
        if title.trim().len()==0{
          return Err(errors::Error::new(None,Some("Ivalid post's title! It can not be empty!".to_string()),errors::ErrorTypes::ValidationError));
        }

        Ok(Title(title.to_owned()))
    }
}

#[derive(Debug)]
pub struct NewPost{
    pub title: Title,
    pub description: Option<String>,
    pub user_id:uuid::Uuid
}

impl NewPost{
  pub fn parse(title:&str,description:&Option<String>,user_id:uuid::Uuid)->Result<NewPost,errors::Error>{
    let title=Title::parse(title)?;
    Ok(
      NewPost { title, description: description.to_owned(), user_id: user_id.to_owned() }
    )

  }
    pub fn build(&self) -> Post{
    tracing::info!("Converting data to DB Post!");
      Post { id: uuid::Uuid::new_v4(), user_id:self.user_id, important:false,title: self.title.0.clone(), description: self.description.clone(), created_at: OffsetDateTime::now_utc() }  
    }
}
