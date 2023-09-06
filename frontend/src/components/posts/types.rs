use serde::Serialize;

#[derive(Debug,Clone,Default,Serialize)]
pub struct FormData{
    pub title:String,
    pub description:Option<String>
}
