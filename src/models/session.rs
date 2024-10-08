use std::fmt::Display;

use crate::models::user;
use ::chrono::{Local, TimeDelta};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{self};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct SessionId(Uuid);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Session is expired")]
    ExpiredSession(Session),
}

impl SessionId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
    pub fn get_value(&self) -> &Uuid {
        &self.0
    }
}

impl Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<Uuid> for SessionId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<&SessionId> for Uuid {
    fn from(value: &SessionId) -> Self {
        value.0
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Session {
    pub id: SessionId,
    pub user_id: user::Id,
    pub exp: chrono::DateTime<Local>,
    pub iat: chrono::DateTime<Local>,
}

impl Session {
    pub fn new(user_id: user::Id, secs_till_expiry: u32) -> Self {
        Self {
            id: SessionId::new(),
            user_id,
            iat: chrono::Local::now(),
            exp: chrono::Local::now()
                .checked_add_signed(TimeDelta::new(secs_till_expiry.into(), 0).unwrap())
                .unwrap(),
        }
    }
    pub fn is_expired(&self) -> bool {
        self.exp.timestamp() <= chrono::Local::now().timestamp()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionTokenClaims {
    pub iat: usize,
    pub exp: usize,
    pub aud: String,
    pub session_id: SessionId,
    pub user_id: user::Id,
}

#[derive(Debug, Clone)]
pub struct SessionFactory {
    jwt_secret: secrecy::Secret<String>,
    aud: String,
    exp_secs: u32,
}

impl SessionFactory {
    pub fn new(jwt_secret: secrecy::Secret<String>, aud: String, exp_secs: u32) -> Self {
        Self {
            jwt_secret,
            aud,
            exp_secs,
        }
    }

    pub fn create_session(&self, user_id: user::Id) -> Session {
        Session::new(user_id, self.exp_secs)
    }

    pub fn create_session_jwt(&self, session: Session) -> Result<String, Error> {
        use jsonwebtoken::Algorithm;
        use jsonwebtoken::Header;

        if session.is_expired() {
            return Err(Error::ExpiredSession(session));
        }

        let session_token_claims = SessionTokenClaims {
            session_id: session.id.clone(),
            user_id: session.user_id,
            iat: chrono::Local::now().timestamp().try_into().unwrap(),
            exp: session.exp.timestamp().try_into().unwrap(),
            aud: self.aud.clone(),
        };

        let header: Header = Header::new(Algorithm::HS384);
        let secret_key = self.jwt_secret.expose_secret().as_bytes();

        let encoded_token = jsonwebtoken::encode(
            &header,
            &session_token_claims,
            &jsonwebtoken::EncodingKey::from_secret(secret_key),
        )
        .unwrap();

        Ok(encoded_token)
    }

    pub fn create_jwt_from_user_id(&self, user_id: user::Id) -> Result<String, Error> {
        self.create_session_jwt(self.create_session(user_id))
    }
    pub fn parse_session_jwt(
        &self,
        token: &str,
    ) -> Result<jsonwebtoken::TokenData<SessionTokenClaims>, jsonwebtoken::errors::Error> {
        use jsonwebtoken::Algorithm;

        let mut validator = jsonwebtoken::Validation::new(Algorithm::HS384);
        validator.set_audience(&[&self.aud]);

        jsonwebtoken::decode::<SessionTokenClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.jwt_secret.expose_secret().as_bytes()),
            &validator,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::SessionFactory;
    use crate::configuration::Settings;
    use crate::models::user;

    #[test]
    fn try_tokenizing_sessions() {
        let Settings {
            jwt_secret,
            database: _,
            host: _,
            port: _,
            app_env: _,
        } = Settings::get_settings().expect("application settings should not error out");

        let session_factory = SessionFactory::new(jwt_secret, "users".into(), 600);
        let session = session_factory.create_session(user::Id::new());

        let jwt_str = session_factory
            .create_session_jwt(session.clone())
            .expect("should return jwt string");

        let token = session_factory
            .parse_session_jwt(&jwt_str)
            .expect("should return session");

        assert_eq!(token.claims.session_id, session.id);
    }
}
