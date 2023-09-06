use actix_web::{dev::Payload, web, FromRequest, HttpMessage, HttpRequest};

use time::OffsetDateTime;
use tracing::instrument;

use crate::errors::{Auth, Error, ErrorTypes};
use crate::{app::AppState, schema::jwt::TokenType};
use std::future::{ready, Ready};

/// The `JwtMiddleware` struct represents a middleware for handling JSON Web Tokens (JWTs) with an
/// associated user ID.
///
/// Properties:
///
/// * `user_id`: The `user_id` property is of type `uuid::Uuid`, which represents a Universally Unique
/// Identifier (UUID). UUIDs are commonly used to uniquely identify entities in a distributed system.
#[derive(Debug)]
pub struct JwtMiddleware {
    pub user_id: uuid::Uuid,
}

impl FromRequest for JwtMiddleware {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    #[instrument(skip_all,name="Check authorization",fields(uri = %req.uri(), method=%req.method()))]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let state = req
            .app_data::<web::Data<AppState>>()
            .expect("Can not get app state data");

        tracing::info!("Get access jwt token from cookies");
        let tokens = match req.cookie("access_token").map(|c| c.value().to_string()) {
            Some(token) => token,
            None => {
                tracing::error!("Access token not found");
                return ready(Err(Error::new(
                    None,
                    Some("Access token not found. Log in first!".into()),
                    ErrorTypes::Auth(Auth::Authorization),
                )));
            }
        };

        let token = match state.jwt.decode(&tokens, TokenType::Access) {
            Ok(c) => c,
            Err(e) => return ready(Err(e)),
        };

        //check if token is valid only if it's not a refresh request
        if req.uri() != "/auth/refresh"
            && OffsetDateTime::now_utc().unix_timestamp() as usize > token.exp
        {
            tracing::error!("Log in timed out");

            return ready(Err(Error::new(
                None,
                Some("Login timed out".into()),
                ErrorTypes::Auth(Auth::Authorization),
            )));
        }

        //insert Uuid to request
        let user_id = uuid::Uuid::parse_str(token.sub.as_str()).unwrap();
        req.extensions_mut()
            .insert::<uuid::Uuid>(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}
