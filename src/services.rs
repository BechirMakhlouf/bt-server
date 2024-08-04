use serde::Serialize;

use crate::{models::session::SessionFactory, repositories::Repositories, types::app_env::AppEnv};

pub mod auth;
pub mod health;
pub mod user_body_fat;
pub mod user_body_measurements;
pub mod user_info;
pub mod user_profile;
pub mod user_settings;
pub mod user_weight;

#[derive(Debug, Clone, Serialize)]
pub struct ResponseError<'a> {
    error: &'a str,
}

impl<'a> ResponseError<'a> {
    pub fn new(error: &'a str) -> Self {
        Self { error }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    _app_env: AppEnv,
    repositories: Repositories,
    session_factory: SessionFactory,
}

impl AppState {
    pub fn new(
        repositories: Repositories,
        session_factory: SessionFactory,
        app_env: AppEnv,
    ) -> Self {
        Self {
            _app_env: app_env,
            repositories,
            session_factory,
        }
    }
}
