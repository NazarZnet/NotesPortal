use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2::Error;
use serde::{Deserialize, Serialize};

use crate::schema::jwt::Jwt;
/// Defining a type alias `DbPool` for a connection pool of `PgConnection` objects.
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
/// The above code defines a struct called `Settings` with two fields: `database` and `auth`.
///
/// Properties:
///
/// * `database`: The `database` property is of type `DbSettings`. It represents the settings for the
/// database connection and configuration.
/// * `auth`: The `auth` property is of type `AuthSettings`.
#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub database: DbSettings,
    pub auth: AuthSettings,
}

/// The `DbSettings` struct represents the settings needed to connect to a database.
///
/// Properties:
///
/// * `username`: The `username` property is a string that represents the username used to authenticate
/// with the database server.
/// * `password`: The `password` property is a string that represents the password for accessing the
/// database.
/// * `port`: The `port` property is of type `u16`, which stands for an unsigned 16-bit integer. It is
/// used to specify the port number for connecting to a database server.
/// * `host`: The `host` property represents the hostname or IP address of the database server.
/// * `database_name`: The `database_name` property is a string that represents the name of the
/// database.
#[derive(Deserialize, Serialize)]
pub struct DbSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

/// The TokenConfig struct represents the configuration for a token, including its key, expiration time,
/// and maximum age.
///
/// Properties:
///
/// * `key`: The `key` property is a string that represents the secret key used for generating and
/// validating tokens. It is typically a long and randomly generated string that should be kept secure.
/// * `exp`: The `exp` property in the `TokenConfig` struct represents the expiration time of the token.
/// It is of type `i64`, which means it is an integer representing the number of seconds since the Unix
/// epoch (January 1, 1970, 00:00:00 UTC)
/// * `maxage`: The `maxage` property in the `TokenConfig` struct represents the maximum age of a token
/// in seconds. It is used to determine the expiration time of a token.
#[derive(Deserialize, Serialize)]
pub struct TokenConfig {
    pub key: String,
    pub exp: i64,
    pub maxage: i64,
}

/// The `AuthSettings` struct represents the configuration settings for access and refresh tokens in
/// Rust.
///
/// Properties:
///
/// * `access`: The `access` property is of type `TokenConfig`.
/// * `refresh`: The `refresh` property is of type `TokenConfig`.
#[derive(Deserialize, Serialize)]
pub struct AuthSettings {
    pub access: TokenConfig,
    pub refresh: TokenConfig,
}

/// The `AppState` struct represents the state of an application and contains a database connection pool
/// and a JSON Web Token (JWT) object.
///
/// Properties:
///
/// * `connection`: DbPool is a connection pool for a database. It allows multiple connections to be
/// created and reused, improving performance and scalability.
/// * `jwt`: The `jwt` property is an instance of the `Jwt` struct. It is likely used for handling JSON
/// Web Tokens (JWT) authentication and authorization in the application.
pub struct AppState {
    pub connection: DbPool,
    pub jwt: Jwt,
}

impl Settings {
    /// The function `get_configuration` retrieves the settings from a YAML configuration file.
    ///
    /// Returns:
    ///
    /// The function `get_configuration()` returns a `Result` type with the following possible outcomes:
    /// - If the configuration is successfully loaded and deserialized into a `Settings` struct, it
    /// returns `Ok(Settings)`.
    /// - If there is an error during the loading or deserialization process, it returns
    /// `Err(config::ConfigError)`.
    pub fn get_configuration() -> Result<Settings, config::ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");

        //make sure directory configuration exists
        let configuration_directory = base_path.join("configuration");

        let settings = config::Config::builder()
            .add_source(config::File::from(
                configuration_directory.join("config.yaml"),
            ))
            .build()?;

        settings.try_deserialize::<Settings>()
    }

    /// The function creates an application state by obtaining a database connection and initializing a
    /// JWT object.
    ///
    /// Returns:
    ///
    /// a Result type, which can either be Ok(AppState) if the function executes successfully, or
    /// Err(Error) if there is an error.
    pub fn create_app_state(&self) -> Result<AppState, Error> {
        let connection = self.database.get_connection_pool()?;
        Ok(AppState {
            connection,
            jwt: Jwt::new(&self.auth.access, &self.auth.refresh),
        })
    }
}

impl DbSettings {
    /// The function `get_connection_pool` returns a connection pool for a PostgreSQL database.
    ///
    /// Returns:
    ///
    /// a Result type, which can either be Ok(DbPool) or Err(Error).
    fn get_connection_pool(&self) -> Result<DbPool, Error> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        );

        let manager = ConnectionManager::<PgConnection>::new(url);

        r2d2::Pool::builder().test_on_check_out(true).build(manager)
    }
}
