#![allow(unused)]
use std::fmt::Display;

use secrecy::ExposeSecret;
use serde::Deserialize;
use uuid::Uuid;

/// Email Type
#[derive(Debug, Deserialize)]
pub struct Email(String);

#[derive(Debug, Deserialize)]
pub struct Password(secrecy::Secret<String>);

#[derive(Debug, Deserialize)]
pub struct HashedPassword(String);

#[derive(Debug, Deserialize, Default)]
pub struct UserId(uuid::Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn get_value(&self) -> &Uuid {
        &self.0
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<&UserId> for Uuid {
    fn from(value: &UserId) -> Self {
        value.0
    }
}

impl Email {
    pub fn parse(email: &str) -> Result<Self, &str> {
        let is_valid = validator::ValidateEmail::validate_email(&email);
        match is_valid {
            true => Ok(Self(email.into())),
            false => Err("Invalid Email"),
        }
    }
    pub fn from_trusted_str(email: &str) -> Self {
        Self(email.to_string())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Password {
    pub fn parse(password: &str) -> Result<Self, &str> {
        use validator::*;
        let is_valid = ValidateLength::validate_length(&password, Some(6), Some(64), None);

        match is_valid {
            true => Ok(Self(secrecy::Secret::new(password.into()))),
            false => Err("Invalid Password"),
        }
    }
    pub fn hash_with_salt(&self) -> (HashedPassword, String) {
        use bcrypt::Version;
        let hash_result = bcrypt::hash_with_salt::<&str>(self.as_str(), 6, rand::random()).unwrap();
        let hashed_password = hash_result.format_for_version(Version::TwoB);
        let salt = hash_result.get_salt();
        drop(hash_result);
        (HashedPassword::from_trusted_str(&hashed_password), salt)
    }
    pub fn as_str(&self) -> &str {
        self.0.expose_secret()
    }
}

impl HashedPassword {
    pub fn parse(password: &str) -> Result<Self, &str> {
        Ok(Self(password.into()))
    }

    pub fn from_trusted_str(hashed_password: &str) -> Self {
        Self(hashed_password.to_string())
    }

    // pub fn as_str(&self) -> &str {
    //     &self.0
    // }
}
impl Display for HashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub email: Email,
    pub password: Password,
}

impl NewUser {
    pub fn new(email: &str, password: &str) -> Result<Self, String> {
        Ok(NewUser {
            email: Email::parse(email)?,
            password: Password::parse(password)?,
        })
    }
}

pub struct User {
    pub email: Email,
    pub hashed_password: HashedPassword,
    pub id: UserId,
}
