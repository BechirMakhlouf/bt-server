use std::str::FromStr;

use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use validator::ValidateRegex;

use super::user;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    #[error("Invalid birthday date: {0}")]
    InvalidBirthdayNaiveDate(chrono::NaiveDate),

    #[error("Invalid birthday date: {0}")]
    InvalidBirthdayString(String),

    #[error("Invalid username: {0}")]
    InvalidUsername(String),

    #[error("Invalid gender: {0}")]
    InvalidGender(String),

    #[error("Invalid user id: {0}")]
    InvalidUserId(String),
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
    Unknown,
}

impl From<Gender> for String {
    fn from(gender: Gender) -> Self {
        match gender {
            Gender::Male => "male".into(),
            Gender::Female => "female".into(),
            Gender::Other => "other".into(),
            Gender::Unknown => "unknown".into(),
        }
    }
}

impl From<Gender> for &str {
    fn from(gender: Gender) -> Self {
        match gender {
            Gender::Male => "male",
            Gender::Female => "female",
            Gender::Other => "other",
            Gender::Unknown => "unknown",
        }
    }
}

impl TryFrom<&str> for Gender {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            "other" => Ok(Gender::Other),
            "unknown" => Ok(Gender::Unknown),
            invalid_gender => Err(Error::InvalidGender(invalid_gender.into())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[repr(transparent)]
pub struct Username(String);

impl TryFrom<String> for Username {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let username_regex = regex::Regex::new(r"^(?i)[a-z0-9_]{3,12}$").unwrap();

        match ValidateRegex::validate_regex(&value, username_regex) {
            true => Ok(Self(value)),
            false => Err(Error::InvalidUsername(value)),
        }
    }
}

impl TryFrom<&str> for Username {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let username_regex = regex::Regex::new(r"^(?i)[a-z0-9_]{3,12}$").unwrap();

        match ValidateRegex::validate_regex(&value, username_regex) {
            true => Ok(Self(value.to_string())),
            false => Err(Error::InvalidUsername(value.to_string())),
        }
    }
}

impl From<Username> for String {
    fn from(value: Username) -> Self {
        value.0
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, sqlx::Type)]
#[sqlx(transparent)]
pub struct Birthday(NaiveDate);

impl TryFrom<NaiveDate> for Birthday {
    type Error = Error;

    fn try_from(date: chrono::NaiveDate) -> Result<Self, Self::Error> {
        let age = chrono::Utc::now().date_naive().years_since(date);

        if age.is_none() {
            return Err(Error::InvalidBirthdayNaiveDate(date));
        }
        match age.unwrap() {
            1..=124 => Ok(Self(date)),
            _ => Err(Error::InvalidBirthdayNaiveDate(date)),
        }
    }
}

impl TryFrom<&str> for Birthday {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match NaiveDate::from_str(value) {
            Ok(naive_date) => Ok(Self::try_from(naive_date)?),
            Err(_) => Err(Error::InvalidBirthdayString(value.into())),
        }
    }
}
impl From<Birthday> for NaiveDate {
    fn from(value: Birthday) -> Self {
        value.0
    }
}

impl Birthday {
    pub fn get_age(&self) -> u8 {
        Utc::now()
            .date_naive()
            .years_since(self.0)
            .unwrap()
            .try_into()
            .unwrap()
    }
}

impl From<&Birthday> for NaiveDate {
    fn from(value: &Birthday) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: user::Id,
    pub username: Username,
    pub gender: Gender,
    pub birthday: Birthday,
}

impl UserInfo {
    pub fn new(
        user_id: user::Id,
        username: String,
        gender: Gender,
        birthday: NaiveDate,
    ) -> Result<Self, Error> {
        Ok(Self {
            user_id,
            birthday: Birthday::try_from(birthday)?,
            gender,
            username: Username::try_from(username)?,
        })
    }

    pub fn try_from_strs(
        user_id: &str,
        username: &str,
        gender: &str,
        birthday: &str,
    ) -> Result<Self, Error> {
        Ok(Self {
            birthday: birthday.try_into()?,
            gender: gender.try_into()?,
            username: username.try_into()?,
            user_id: user_id
                .try_into()
                .map_err(|_| Error::InvalidUserId(user_id.to_string()))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Birthday;
    use chrono::NaiveDate;

    #[test]
    fn test_age() {
        let b1 = Birthday::try_from(NaiveDate::from_ymd_opt(2001, 7, 22).unwrap())
            .expect("this should work");

        assert_eq!(b1.get_age(), 23);
    }
}
