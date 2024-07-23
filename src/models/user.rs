use std::fmt::Display;

use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Email Type
#[derive(Debug, Deserialize)]
pub struct Email(String);

#[derive(Debug, Deserialize)]
pub struct Password(secrecy::Secret<String>);

#[derive(Debug, Deserialize)]
pub struct HashedPassword(String);

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Default,
    Clone,
    PartialEq,
    // sqlx::Type,
    sqlx::encode::Encode,
    sqlx::decode::Decode,
)]
#[sqlx(transparent)]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn get_value(&self) -> &Uuid {
        &self.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<Id> for Uuid {
    fn from(value: Id) -> Self {
        value.0
    }
}
impl From<&Id> for Uuid {
    fn from(value: &Id) -> Self {
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

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
    pub fn hash_with_salt(&self) -> HashedPassword {
        use bcrypt::Version;
        let hash_result = bcrypt::hash_with_salt::<&str>(self.as_str(), 6, rand::random()).unwrap();
        let hashed_password = hash_result.format_for_version(Version::TwoB);
        // let salt = hash_result.get_salt();
        drop(hash_result);
        HashedPassword::from_trusted_str(&hashed_password)
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

    pub fn compare_with(&self, password: &Password) -> bool {
        bcrypt::verify(password.as_str(), self.as_str()).unwrap()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
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

#[derive(Debug)]
pub struct User {
    pub email: Email,
    pub hashed_password: HashedPassword,
    pub id: Id,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_passwords() {
        let my_password = Password::parse("pass_pass").expect("errored on a valid password");
        let hashed_password = my_password.hash_with_salt();

        let re = hashed_password.compare_with(&my_password);

        assert!(re, "Password verification should be correct.");
    }
}
