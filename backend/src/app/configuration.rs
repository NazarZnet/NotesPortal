use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2::Error;
use serde::{Deserialize, Serialize};

use crate::schema::jwt::Jwt;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub database: DbSettings,
    pub auth: AuthSettings,
}

#[derive(Deserialize, Serialize)]
pub struct DbSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct TokenConfig {
    pub key: String,
    pub exp: i64,
    pub maxage: i64,
}

#[derive(Deserialize, Serialize)]
pub struct AuthSettings {
    pub access: TokenConfig,
    pub refresh: TokenConfig,
}

pub struct AppState {
    pub connection: DbPool,
    pub jwt: Jwt,
}

impl Settings {
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

    pub fn create_app_state(&self) -> Result<AppState, Error> {
        let connection = self.database.get_connection_pool()?;
        Ok(AppState {
            connection,
            jwt: Jwt::new(&self.auth.access, &self.auth.refresh),
        })
    }
}

impl DbSettings {
    fn get_connection_pool(&self) -> Result<DbPool, Error> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        );

        let manager = ConnectionManager::<PgConnection>::new(url);

        r2d2::Pool::builder().test_on_check_out(true).build(manager)
    }
}
