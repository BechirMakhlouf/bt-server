#![allow(dead_code)]
use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    models::{
        session::{self, Session, SessionFactory},
        user::{self, Email, UserCredentials},
    },
    repositories::UserRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("There is no user with email `{0}`.")]
    EmailNotFound(Email),
    #[error("There is no user with the provided credentials.")]
    WrongCredentials,

    #[error("Email: `{0}` already used.")]
    UsedEmail(Email),

    #[error("Unknown auth error.")]
    Unknown,
}

#[derive(Debug)]
pub struct Authenticator {
    user_repo: UserRepository,
    session_factory: SessionFactory,
}

impl Authenticator {
    pub fn new(db_pool: Arc<Pool<Postgres>>, jwt_secret: secrecy::Secret<String>) -> Self {
        Self {
            user_repo: UserRepository::new(db_pool),
            session_factory: SessionFactory::new(jwt_secret, "users".into(), 600),
        }
    }

    pub async fn auth_user_email_password(
        &self,
        email: user::Email,
        password: user::Password,
    ) -> Result<session::Session, Error> {
        let user = self.user_repo.get_by_email(&email).await;

        if user.is_err() {
            return Err(Error::Unknown);
        }
        let user = user.unwrap();

        if user.is_none() {
            return Err(Error::WrongCredentials);
        }
        let user = user.unwrap();
        if user.encrypted_password.compare_with(&password) {
            Ok(session::Session::new(user.id, 600))
        } else {
            Err(Error::WrongCredentials)
        }
    }

    pub async fn sign_up_user(&self, user_credentials: UserCredentials) -> Result<user::Id, Error> {
        let add_user_result = self.user_repo.add(&user_credentials).await;

        if let Ok(user_id) = add_user_result {
            return Ok(user_id);
        }
        let error = add_user_result.unwrap_err();

        if let sqlx::Error::Database(err) = error {
            match err.kind() {
                sqlx::error::ErrorKind::UniqueViolation => {
                    return Err(Error::UsedEmail(user_credentials.email))
                }
                _ => return Err(Error::Unknown),
            }
        }
        Err(Error::Unknown)
    }

    pub fn create_session_token(&self, session: Session) -> Result<String, session::Error> {
        if session.is_expired() {
            return Err(session::Error::ExpiredSession(session));
        }

        Ok(self.session_factory.create_session_jwt(session).unwrap())
    }
}
