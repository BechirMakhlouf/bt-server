#![allow(unused)]
use secrecy::ExposeSecret;
use uuid::Uuid;

/// Email Type
#[derive(Debug, serde::Deserialize)]
pub struct Email(String);

#[derive(Debug, serde::Deserialize)]
pub struct Password(secrecy::Secret<String>);

#[derive(Debug, serde::Deserialize)]
pub struct HashedPassword(String);

#[derive(Debug, serde::Deserialize)]
pub struct UserId(uuid::Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
    pub fn get_value(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}
impl Into<Uuid> for UserId {
    fn into(self) -> Uuid {
        self.0
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
}

#[derive(Debug, serde::Deserialize)]
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
