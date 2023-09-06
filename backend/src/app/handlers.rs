use actix_web::{get, post, web, HttpResponse};

use common::{PostsFormData, PostsUpdateForm};

use crate::{app::AppState, auth::JwtMiddleware, db::db_get_posts};
use crate::{
    db::{db_add_post, db_update_post},
    errors,
    schema::post::NewPost,
};
use tracing::instrument;

#[get("/posts")]
#[instrument(skip_all, name = "Get all posts")]
async fn get_posts(
    state: web::Data<AppState>,
    _: JwtMiddleware,
) -> Result<HttpResponse, errors::Error> {
    let connection = state.connection.clone();

    let db_posts = web::block(move || db_get_posts(&connection)).await??;
    Ok(HttpResponse::Ok().json(db_posts))
}

#[post("/posts")]
#[instrument(skip_all, name = "Create new post")]
async fn add_post(
    state: web::Data<AppState>,
    auth: JwtMiddleware,
    data: web::Json<PostsFormData>,
) -> Result<HttpResponse, errors::Error> {
    let new_post = NewPost::parse(&data.title, &data.description, auth.user_id)?.build();
    let connection = state.connection.clone();

    let db_posts = web::block(move || db_add_post(new_post, &connection)).await??;
    Ok(HttpResponse::Ok().json(db_posts))
}

#[post("/posts/update")]
#[instrument(skip_all, name = "Update post's important field")]
async fn update_posts(
    state: web::Data<AppState>,
    _: JwtMiddleware,
    data: web::Json<PostsUpdateForm>,
) -> Result<HttpResponse, errors::Error> {
    let update_data = data.clone();
    let connection = state.connection.clone();

    let db_post = web::block(move || db_update_post(update_data, &connection)).await??;
    Ok(HttpResponse::Ok().json(db_post))
}
