use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not a positive, non-zero number: {0}")]
    NegativeFloat(f64),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct PositiveNonZeroF64(f64);

impl TryFrom<f64> for PositiveNonZeroF64 {
    type Error = Error;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value > 0.0 {
            Ok(Self(value))
        } else {
            Err(Error::NegativeFloat(value))
        }
    }
}

impl From<PositiveNonZeroF64> for f64 {
    fn from(value: PositiveNonZeroF64) -> Self {
        value.0
    }
}

pub fn to_some_f64(n: Option<PositiveNonZeroF64>) -> Option<f64> {
    n.map(|n| n.into())
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PosNonZeroF32(f32);

impl TryFrom<f32> for PosNonZeroF32 {
    type Error = Error;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value > 0.0 {
            Ok(Self(value))
        } else {
            Err(Error::NegativeFloat(value.into()))
        }
    }
}

impl From<PosNonZeroF32> for f32 {
    fn from(value: PosNonZeroF32) -> Self {
        value.0
    }
}
impl PosNonZeroF32 {
    pub fn from_trusted(n: f32) -> Self {
        Self(n)
    }
}
pub fn to_optional_f32(n: Option<PosNonZeroF32>) -> Option<f32> {
    n.map(|n| n.into())
}

pub fn to_opt_pos_f32(n: Option<f32>) -> Option<PosNonZeroF32> {
    Some(PosNonZeroF32::from_trusted(n?))
}
