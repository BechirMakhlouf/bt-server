#![allow(dead_code)]
use std::fmt::Display;

use super::user::UserId;
use ::chrono::{Days, Local};
use jsonwebtoken::{Algorithm, Header, TokenData};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{self};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct SessionId(Uuid);

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
    pub user_id: UserId,
    pub exp: chrono::DateTime<Local>,
    pub iat: chrono::DateTime<Local>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SessionTokenClaims {
    iat: usize,
    exp: usize,
    aud: String,
    session_id: SessionId,
}

impl Session {
    pub fn new(user_id: UserId, days_until_exp: Days) -> Self {
        Self {
            id: SessionId::new(),
            user_id,
            iat: chrono::Local::now(),
            exp: chrono::Local::now()
                .checked_add_days(days_until_exp)
                .unwrap(),
        }
    }
}
