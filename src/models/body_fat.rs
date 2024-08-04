use std::f32;

use ::chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

use super::user;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("body fat is invalid: {0}")]
    InvalidBodyFat(f32),

    #[error("Invalid date for bodyfat: {0}")]
    InvalidBodyFatDate(chrono::NaiveDate),
}

pub type Result<T> = core::result::Result<T, Error>;

//TODO: weightDate interval struct

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd)]
pub struct BodyFat(f32);

impl TryFrom<f32> for BodyFat {
    type Error = Error;
    fn try_from(value: f32) -> Result<Self> {
        match value {
            0.0..=100.0 => Ok(Self(value)),
            _ => Err(Error::InvalidBodyFat(value)),
        }
    }
}

impl From<BodyFat> for f32 {
    fn from(value: BodyFat) -> Self {
        value.0
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd)]
pub struct BodyFatDate(chrono::NaiveDate);

impl BodyFatDate {
    pub fn parse(date: chrono::NaiveDate) -> Result<Self> {
        if date > chrono::Utc::now().date_naive() {
            return Err(Error::InvalidBodyFatDate(date));
        }

        Ok(Self(date))
    }
}

impl TryFrom<NaiveDate> for BodyFatDate {
    type Error = Error;

    fn try_from(value: chrono::NaiveDate) -> Result<Self> {
        if value > chrono::Utc::now().date_naive() {
            return Err(Error::InvalidBodyFatDate(value));
        }
        Ok(Self(value))
    }
}

impl From<BodyFatDate> for NaiveDate {
    fn from(value: BodyFatDate) -> Self {
        value.0
    }
}

#[derive(Debug, Clone)]
pub struct UserBodyFat {
    pub user_id: user::Id,
    pub body_fat: BodyFat,
    pub date: BodyFatDate,
}

impl UserBodyFat {
    pub fn new(
        user_id: user::Id,
        body_fat: f32,
        date_at: chrono::NaiveDate,
    ) -> Result<Self, Error> {
        Ok(Self {
            user_id,
            body_fat: body_fat.try_into()?,
            date: date_at.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::BodyFat;

    #[test]
    fn test_body_fat_equality_ord() {
        assert_eq!(
            BodyFat::try_from(20.0).unwrap(),
            BodyFat::try_from(20.0).unwrap(),
            "Values should be equal."
        );

        assert_ne!(
            BodyFat::try_from(20.0).unwrap(),
            BodyFat::try_from(20.1).unwrap(),
            "Values should not be equal."
        );

        assert!(BodyFat::try_from(20.0).unwrap() < BodyFat::try_from(22.0).unwrap())
    }
}
