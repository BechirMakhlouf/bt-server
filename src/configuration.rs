use secrecy::Secret;
use sqlx::{Database, Pool};
use url::Url;

#[derive(Debug, serde::Deserialize)]
pub struct EnvVars {
    database_url: url::Url,
    jwt_secret: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseSettings {
    database_url: Url,
}

#[derive(Debug, serde::Deserialize)]
pub struct ApplicationSettings {
    pub database: DatabaseSettings,
    pub jwt_secret: Secret<String>,
}

impl ApplicationSettings {
    pub fn get_settings() -> Result<Self, envy::Error> {
        let EnvVars {
            database_url,
            jwt_secret,
        } = match envy::from_env::<EnvVars>() {
            Ok(env_vars) => env_vars,
            Err(e) => return Err(e),
        };

        let database = DatabaseSettings { database_url };
        Ok(Self {
            database,
            jwt_secret: Secret::new(jwt_secret),
        })
    }
}

impl DatabaseSettings {
    pub async fn get_db_pool<T: Database>(&self) -> Result<Pool<T>, sqlx::error::Error> {
        Pool::<T>::connect(self.database_url.as_str()).await
    }
}
