use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd)]
pub struct PastNaiveDate(NaiveDate);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid PastNaiveDate: {0}")]
    InvalidPastNaiveDate(chrono::NaiveDate),
}

impl TryFrom<NaiveDate> for PastNaiveDate {
    type Error = Error;

    fn try_from(value: chrono::NaiveDate) -> Result<Self, Self::Error> {
        if value > chrono::Utc::now().date_naive() {
            return Err(Error::InvalidPastNaiveDate(value));
        }
        Ok(Self(value))
    }
}

impl From<PastNaiveDate> for NaiveDate {
    fn from(value: PastNaiveDate) -> Self {
        value.0
    }
}
