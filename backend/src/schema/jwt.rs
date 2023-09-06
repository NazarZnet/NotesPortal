use crate::app::TokenConfig;
use crate::errors::{Auth, Error, ErrorTypes};
use actix_web::{HttpMessage, HttpRequest};

use actix_web::cookie::time::Duration as ActixWebDuration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use time::{Duration, OffsetDateTime};
use tracing::instrument;

#[derive(Debug, Serialize, Deserialize, Clone)]
/// The `TokenClaims` struct represents the claims of a token, including the subject and expiration
/// time.
///
/// Properties:
///
/// * `sub`: The `sub` property in the `TokenClaims` struct represents the subject of the token. It
/// typically contains a unique identifier for the user or entity that the token represents.
/// * `exp`: The `exp` property in the `TokenClaims` struct represents the expiration time of the token.
/// It is of type `usize`, which means it stores a non-negative integer value. The value represents the
/// number of seconds since January 1, 1970 (also known as the Unix timestamp)
pub struct TokenClaims {
    pub sub: String,
    pub exp: usize,
}

impl TokenClaims {
    /// The function creates a new token with a specified subject and expiration time.
    ///
    /// Arguments:
    ///
    /// * `sub`: The `sub` parameter is a string that represents the subject of the token. It typically
    /// identifies the user or entity that the token is being issued for.
    /// * `time`: The `time` parameter is of type `Duration`. It represents a duration of time, such as
    /// 5 minutes, 1 hour, etc. It is used to calculate the expiration time of the token.
    ///
    /// Returns:
    ///
    /// The `new` function returns an instance of the `TokenClaims` struct.
    pub fn new(sub: String, time: Duration) -> Self {
        //create token
        let now = OffsetDateTime::now_utc();
        let exp = (now + time).unix_timestamp() as usize;
        TokenClaims { sub, exp }
    }
}
/// The `TokenSettings` struct represents the settings for token encoding and decoding in Rust.
///
/// Properties:
///
/// * `decode_key`: A DecodingKey is used to decode a JSON Web Token (JWT). It is typically a secret key
/// or a public key used to verify the authenticity of the token.
/// * `encode_key`: The `encode_key` property is of type `EncodingKey`. It is used to encode (or sign)
/// the token, which means it is used to generate a digital signature for the token. This signature is
/// used to verify the authenticity and integrity of the token when it is received by the server.
/// * `exp`: The `exp` property represents the expiration time for the token. It is of type `Duration`,
/// which is a struct that represents a length of time. This property determines how long the token will
/// be valid before it expires.
/// * `maxage`: The `maxage` property is of type `ActixWebDuration` and is used to specify the maximum
/// age of a token. It determines how long a token remains valid before it expires.
pub struct TokenSettings {
    decode_key: DecodingKey,
    encode_key: EncodingKey,
    pub exp: Duration,
    pub maxage: ActixWebDuration,
}

/// The `Jwt` struct represents a JSON Web Token with access and refresh token settings.
///
/// Properties:
///
/// * `access`: The `access` property is of type `TokenSettings`. It represents the settings for the
/// access token in the JWT (JSON Web Token).
/// * `refresh`: The `refresh` property is of type `TokenSettings`.
pub struct Jwt {
    pub access: TokenSettings,
    pub refresh: TokenSettings,
}

pub enum TokenType {
    Refresh,
    Access,
}

impl Jwt {
    /// The function creates a new instance of a Jwt struct with access and refresh token
    /// configurations.
    ///
    /// Arguments:
    ///
    /// * `access_config`: The `access_config` parameter is a `TokenConfig` struct that contains the
    /// configuration settings for the access token. It includes the following fields:
    /// * `refresh_config`: The `refresh_config` parameter is a `TokenConfig` struct that contains the
    /// configuration settings for the refresh token. It includes the following fields:
    ///
    /// Returns:
    ///
    /// The `new` function returns an instance of the `Jwt` struct.
    pub fn new(access_config: &TokenConfig, refresh_config: &TokenConfig) -> Self {
        Jwt {
            access: TokenSettings {
                decode_key: DecodingKey::from_secret(access_config.key.as_bytes()),
                encode_key: EncodingKey::from_secret(access_config.key.as_bytes()),
                exp: Duration::minutes(access_config.exp),
                maxage: ActixWebDuration::new(60 * access_config.maxage, 0),
            },
            refresh: TokenSettings {
                decode_key: DecodingKey::from_secret(refresh_config.key.as_bytes()),
                encode_key: EncodingKey::from_secret(refresh_config.key.as_bytes()),
                exp: Duration::minutes(refresh_config.exp),
                maxage: ActixWebDuration::new(60 * refresh_config.maxage, 0),
            },
        }
    }

    /// The `encode` function encodes a JWT token using the provided claims and token type.
    ///
    /// Arguments:
    ///
    /// * `token`: The `token` parameter is of type `TokenClaims`, which represents the claims to be
    /// encoded in the JWT token. It contains information such as the subject, issuer, expiration time,
    /// and any custom claims.
    /// * `token_type`: The `token_type` parameter is an enum called `TokenType`. It is used to
    /// determine whether the token being encoded is an access token or a refresh token.
    ///
    /// Returns:
    ///
    /// The function `encode` returns a `Result` containing a `String` if the encoding is successful, or
    /// an `Error` if there is an error during the encoding process.
    pub fn encode(&self, token: &TokenClaims, token_type: TokenType) -> Result<String, Error> {
        tracing::info!("JWT token encoding");
        let token =
            match token_type {
                TokenType::Access => {
                    jsonwebtoken::encode(&Header::default(), token, &self.access.encode_key)
                        .map_err(|e| {
                            tracing::error!("Error creating new access token");
                            Error::new(
                                Some(e.to_string()),
                                Some("Can not create token".into()),
                                ErrorTypes::JwtError,
                            )
                        })?
                }
                TokenType::Refresh => {
                    jsonwebtoken::encode(&Header::default(), token, &self.refresh.encode_key)
                        .map_err(|e| {
                            tracing::error!("Error creating new refresh token");
                            Error::new(
                                Some(e.to_string()),
                                Some("Can not create token".into()),
                                ErrorTypes::JwtError,
                            )
                        })?
                }
            };

        Ok(token)
    }

    /// The `decode` function decodes a JWT token and returns the token claims, while the `refresh`
    /// function retrieves a JWT refresh token from cookies.
    ///
    /// Arguments:
    ///
    /// * `claim`: The `claim` parameter is a string that represents the JWT token that needs to be
    /// decoded. It contains the encoded information about the user or entity associated with the token.
    /// * `token_type`: The `token_type` parameter is an enum called `TokenType` which represents the
    /// type of token being decoded. It can have two possible values: `TokenType::Access` or
    /// `TokenType::Refresh`.
    ///
    /// Returns:
    ///
    /// The `decode` function returns a `Result<TokenClaims, Error>`. The `refresh` function returns a
    /// `Result<uuid::Uuid, Error>`.
    pub fn decode(&self, claim: &str, token_type: TokenType) -> Result<TokenClaims, Error> {
        tracing::info!("JWT token decoding");
        let token = match token_type {
            TokenType::Access => jsonwebtoken::decode::<TokenClaims>(
                claim,
                &self.access.decode_key,
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            )
            .map_err(|e| {
                Error::new(
                    Some(e.to_string()),
                    Some("Can not decode token".into()),
                    ErrorTypes::JwtError,
                )
            })?,
            TokenType::Refresh => jsonwebtoken::decode::<TokenClaims>(
                claim,
                &self.refresh.decode_key,
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            )
            .map_err(|e| {
                Error::new(
                    Some(e.to_string()),
                    Some("Can not decode token".into()),
                    ErrorTypes::JwtError,
                )
            })?,
        };
        Ok(token.claims)
    }

    /// The `refresh` function in Rust is used to refresh a JWT token by decoding and validating it, and
    /// returning the user ID associated with the token.
    ///
    /// Arguments:
    ///
    /// * `req`: HttpRequest - The HTTP request object that contains information about the incoming
    /// request, such as headers, cookies, and query parameters. It is used to retrieve the refresh
    /// token from the cookies.
    ///
    /// Returns:
    ///
    /// The function `refresh` returns a `Result` with the type `uuid::Uuid` as the success variant and
    /// `Error` as the error variant.
    #[instrument(skip_all, name = "Refresh jwt token")]
    pub fn refresh(&self, req: &HttpRequest) -> Result<uuid::Uuid, Error> {
        tracing::info!("Get jwt refresh token from cookies");

        let tokens = match req.cookie("refresh_token").map(|c| c.value().to_string()) {
            Some(token) => token,
            None => {
                tracing::error!("JWT refresh token not found");
                return Err(Error::new(
                    None,
                    Some("Refresh jwt token not found. Log in first!".into()),
                    ErrorTypes::Auth(Auth::Authorization),
                ));
            }
        };

        let token = match self.decode(&tokens, TokenType::Refresh) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Invalid refresh jwt token");

                return Err(Error::new(
                    Some(e.to_string()),
                    Some("Invalid refresh jwt tokens".into()),
                    ErrorTypes::JwtError,
                ));
            }
        };

        //check if token is valid
        if OffsetDateTime::now_utc().unix_timestamp() as usize > token.exp {
            tracing::error!("Refresh token timed out");

            return Err(Error::new(
                None,
                Some("Refresh token timed out".into()),
                ErrorTypes::Auth(Auth::Authorization),
            ));
        }

        //Get user id
        let req_ext = req.extensions();

        let user_id = req_ext.get::<uuid::Uuid>().ok_or(Error::new(
            None,
            Some("Can not find user's id".into()),
            ErrorTypes::Auth(Auth::Authentication),
        ))?;

        Ok(*user_id)
    }
}
