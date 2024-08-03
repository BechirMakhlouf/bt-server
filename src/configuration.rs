#![allow(unused)]
use secrecy::Secret;
use sqlx::{Database, Pool};
use url::Url;

use crate::types::app_env::AppEnv;

#[derive(Debug, serde::Deserialize)]
pub struct EnvVars {
    database_url: url::Url,
    port: u16,
    jwt_secret: String,
    host: String,
    app_env: AppEnv,
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseSettings {
    pub database_url: Url,
}

#[derive(Debug, serde::Deserialize)]
pub struct ServerSettings {
    pub database: DatabaseSettings,
    pub host: String,
    pub port: u16,
    pub jwt_secret: Secret<String>,
    pub app_env: AppEnv,
}

impl ServerSettings {
    pub fn get_settings() -> Result<Self, envy::Error> {
        let _ = dotenvy::dotenv();

        let EnvVars {
            database_url,
            jwt_secret,
            host,
            port,
            app_env,
        } = match envy::from_env::<EnvVars>() {
            Ok(env_vars) => env_vars,
            Err(e) => return Err(e),
        };

        let database = DatabaseSettings { database_url };

        Ok(Self {
            host,
            port,
            database,
            app_env,
            jwt_secret: Secret::new(jwt_secret),
        })
    }
}

impl DatabaseSettings {
    pub async fn get_db_pool<T: Database>(&self) -> Result<Pool<T>, sqlx::error::Error> {
        Pool::<T>::connect(self.database_url.as_str()).await
    }
}
