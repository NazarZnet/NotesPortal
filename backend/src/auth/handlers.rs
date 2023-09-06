use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpRequest, HttpResponse, Responder,
};

use crate::errors;
use crate::{
    app::AppState,
    auth::JwtMiddleware,
    db::{db_add_user, db_check_user, db_find_user},
    schema::{
        jwt::{TokenClaims, TokenType},
        user::NewUser,
    },
};
use common::UserFormData;
use serde_json::json;
use time::Duration;
use tracing::instrument;

#[post("/auth/signup")]
#[instrument(skip(state), name = "Sign up user")]
pub async fn signup_user(
    data: web::Json<UserFormData>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, errors::Error> {
    let new_user = NewUser::parse(&data.username, &data.password)?.build()?;

    let db_user = web::block(move || db_add_user(new_user, &state.connection)).await??;

    Ok(HttpResponse::Ok().json(db_user))
}

#[post("/auth/login")]
#[instrument(skip(state), name = "User log in")]
async fn login_user(
    data: web::Json<UserFormData>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, errors::Error> {
    let new_user = NewUser::parse(&data.username, &data.password)?;
    let connection = state.connection.clone();
    let db_user = web::block(move || db_check_user(new_user, &connection)).await??;

    let access_token = state.jwt.encode(
        &TokenClaims::new(db_user.id.to_string(), state.jwt.access.exp),
        TokenType::Access,
    )?;

    let refresh_token = state.jwt.encode(
        &TokenClaims::new(db_user.id.to_string(), state.jwt.refresh.exp),
        TokenType::Refresh,
    )?;

    let aceess_cookie = Cookie::build("access_token", access_token.to_owned())
        .path("/")
        .max_age(state.jwt.access.maxage)
        .http_only(true)
        .finish();

    let refresh_cookie = Cookie::build("refresh_token", refresh_token.to_owned())
        .path("/")
        .max_age(state.jwt.refresh.maxage)
        .http_only(true)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(aceess_cookie)
        .cookie(refresh_cookie)
        .json(db_user))
}

#[get("/auth/refresh")]
#[instrument(skip_all, name = "User refresh authorization")]
async fn refresh_auth(
    req: HttpRequest,
    state: web::Data<AppState>,
    _: JwtMiddleware,
) -> Result<HttpResponse, errors::Error> {
    //check refresh token and find User's id
    let user_id = state.jwt.refresh(&req)?;

    //check if user exists and Uuid valid
    let connection = state.connection.clone();
    let db_user = web::block(move || db_find_user(user_id, &connection)).await??;

    let new_token = state.jwt.encode(
        &TokenClaims::new(db_user.id.to_string(), Duration::minutes(1)),
        TokenType::Access,
    )?;

    let cookie = Cookie::build("access_token", new_token.to_owned())
        .path("/")
        .max_age(state.jwt.access.maxage)
        .http_only(true)
        .finish();
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "new_access": new_token})))
}

#[get("/auth/logout")]
#[instrument(name = "User logout")]
async fn logout_handler(_: JwtMiddleware) -> impl Responder {
    let refresh_cookie = Cookie::build("refresh_token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    let access_cookie = Cookie::build("access_token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .json(json!({"status": "success"}))
}
