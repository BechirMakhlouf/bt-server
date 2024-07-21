use chrono::NaiveDate;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    models::body_measurements::BodyMeasurementsCm,
    types::positive_non_zero_float::PositiveNonZeroF64,
};

#[derive(Debug)]
pub struct BodyMeasurementsRepository {
    database: Pool<Postgres>,
}

pub fn to_some_f64(n: Option<PositiveNonZeroF64>) -> Option<f64> {
    match n {
        Some(n) => Some(n.into()),
        None => None,
    }
}

impl BodyMeasurementsRepository {
    pub fn add(&self, body_measurement: BodyMeasurementsCm) {
        sqlx::query!(
            "insert into arms (user_id, date_at, left_arm, right_arm) values ($1, $2, $3, $4)",
            Uuid::from(body_measurement.user_id),
            NaiveDate::from(body_measurement.date_at),
            to_some_f64(body_measurement.left_arm),
            to_some_f64(body_measurement.right_arm)
        );
    }
}
