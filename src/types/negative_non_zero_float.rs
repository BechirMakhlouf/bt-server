#[derive(Debug, PartialEq, PartialOrd)]
pub struct PositiveNonZeroF64(f64);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not a positive, non-zero number: {0}")]
    NegativeFloat(f64),
}

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

impl Option<PositiveNonZeroF64> {}
