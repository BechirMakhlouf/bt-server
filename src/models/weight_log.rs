use std::f64;

use ::chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

use super::user;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Weight is invalid: {0}")]
    InvalidWeight(f64),

    #[error("Invalid date for weight: {0}")]
    InvalidWeightDate(chrono::NaiveDate),
}

//TODO: weightDate interval struct

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd)]
pub struct WeightKg(f64);

impl TryFrom<f64> for WeightKg {
    type Error = Error;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        match value {
            5.0..=1000.0 => Ok(Self(value)),
            _ => Err(Error::InvalidWeight(value)),
        }
    }
}

impl From<WeightKg> for f64 {
    fn from(value: WeightKg) -> Self {
        value.0
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd)]
pub struct WeightDate(chrono::NaiveDate);

impl WeightDate {
    pub fn parse(date: chrono::NaiveDate) -> Result<Self, Error> {
        if date > chrono::Utc::now().date_naive() {
            return Err(Error::InvalidWeightDate(date));
        }

        Ok(Self(date))
    }
}

impl TryFrom<NaiveDate> for WeightDate {
    type Error = Error;

    fn try_from(value: chrono::NaiveDate) -> Result<Self, Self::Error> {
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

#[derive(Debug)]
pub struct WeightLog {
    pub user_id: user::Id,
    pub weight_kg: WeightKg,
    pub date: WeightDate,
}

impl WeightLog {
    pub fn new(
        user_id: user::Id,
        weight_kg: f64,
        date_at: chrono::NaiveDate,
    ) -> Result<Self, Error> {
        Ok(Self {
            user_id,
            weight_kg: weight_kg.try_into()?,
            date: date_at.try_into()?,
        })
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
