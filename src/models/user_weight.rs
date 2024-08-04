use std::f32;

use ::chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

use super::user;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Weight is invalid: {0}")]
    InvalidWeight(f32),

    #[error("Invalid date for weight: {0}")]
    InvalidWeightDate(chrono::NaiveDate),
}

pub type Result<T> = core::result::Result<T, Error>;

//TODO: weightDate interval struct

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd)]
pub struct WeightKg(f32);

impl TryFrom<f32> for WeightKg {
    type Error = Error;
    fn try_from(value: f32) -> Result<Self> {
        match value {
            5.0..=1000.0 => Ok(Self(value)),
            _ => Err(Error::InvalidWeight(value)),
        }
    }
}

impl From<WeightKg> for f32 {
    fn from(value: WeightKg) -> Self {
        value.0
    }
}

impl From<&WeightKg> for f32 {
    fn from(value: &WeightKg) -> Self {
        value.0
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd)]
pub struct WeightDate(chrono::NaiveDate);

impl WeightDate {
    pub fn parse(date: chrono::NaiveDate) -> Result<Self> {
        if date > chrono::Utc::now().date_naive() {
            return Err(Error::InvalidWeightDate(date));
        }

        Ok(Self(date))
    }
    pub fn new(date: chrono::NaiveDate) -> Result<Self> {
        if date > chrono::Utc::now().date_naive() {
            return Err(Error::InvalidWeightDate(date));
        }

        Ok(Self(date))
    }
}

impl TryFrom<NaiveDate> for WeightDate {
    type Error = Error;

    fn try_from(value: chrono::NaiveDate) -> Result<Self> {
        if value > chrono::Utc::now().date_naive() {
            return Err(Error::InvalidWeightDate(value));
        }
        Ok(Self(value))
    }
}

impl From<WeightDate> for NaiveDate {
    fn from(value: WeightDate) -> Self {
        value.0
    }
}
impl From<&WeightDate> for NaiveDate {
    fn from(value: &WeightDate) -> Self {
        value.0
    }
}

#[derive(Debug, Serialize)]
pub struct UserWeight {
    pub user_id: user::Id,
    pub weight_kg: WeightKg,
    pub date: WeightDate,
}

impl UserWeight {
    pub fn new(user_id: user::Id, weight_kg: f32, date_at: chrono::NaiveDate) -> Result<Self> {
        Ok(Self {
            user_id,
            weight_kg: weight_kg.try_into()?,
            date: date_at.try_into()?,
        })
    }
    pub fn from_trusted(user_id: user::Id, weight_kg: f32, date_at: chrono::NaiveDate) -> Self {
        Self {
            user_id,
            weight_kg: WeightKg(weight_kg),
            date: WeightDate(date_at),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WeightKg;

    #[test]
    fn test_weight_kg_equality_ord() {
        assert_eq!(
            WeightKg::try_from(20.0).unwrap(),
            WeightKg::try_from(20.0).unwrap(),
            "Values should be equal."
        );

        assert_ne!(
            WeightKg::try_from(20.0).unwrap(),
            WeightKg::try_from(20.1).unwrap(),
            "Values should not be equal."
        );

        assert!(WeightKg::try_from(20.0).unwrap() < WeightKg::try_from(22.0).unwrap())
    }
}
