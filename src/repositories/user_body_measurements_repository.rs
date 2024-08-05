use std::sync::Arc;

use chrono::NaiveDate;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    models::{body_measurements::BodyMeasurementsCm, user},
    types::{
        past_naive_date::PastNaiveDate,
        positive_non_zero_float::{to_opt_pos_f32, to_optional_f32},
    },
};

#[derive(Debug, Clone)]
pub struct UserBodyMeasurementsRepository {
    database: Arc<Pool<Postgres>>,
}

impl UserBodyMeasurementsRepository {
    pub fn new(database: Arc<Pool<Postgres>>) -> Self {
        Self { database }
    }
    pub async fn add(&self, body_measurement: BodyMeasurementsCm) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users_body_measurements_cm 
            (user_id, date_at, left_arm, right_arm, left_thigh, right_thigh, left_wrist, right_wrist, left_calf, right_calf, height, neck, hips, torso, waist)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)",
            Uuid::from(body_measurement.user_id),
            NaiveDate::from(body_measurement.date_at),
            to_optional_f32(body_measurement.left_arm),
            to_optional_f32(body_measurement.right_arm),
            to_optional_f32(body_measurement.left_thigh),
            to_optional_f32(body_measurement.right_thigh),
            to_optional_f32(body_measurement.left_wrist),
            to_optional_f32(body_measurement.right_wrist),
            to_optional_f32(body_measurement.left_calf),
            to_optional_f32(body_measurement.right_calf),
            to_optional_f32(body_measurement.height),
            to_optional_f32(body_measurement.neck),
            to_optional_f32(body_measurement.hips),
            to_optional_f32(body_measurement.torso),
            to_optional_f32(body_measurement.waist)
        ).execute(self.database.as_ref()).await?;
        Ok(())
    }

    pub async fn update(&self, body_measurement: BodyMeasurementsCm) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
        UPDATE users_body_measurements_cm
        SET 
            left_arm = $1,
            right_arm = $2,
            left_thigh = $3,
            right_thigh = $4,
            left_wrist = $5,
            right_wrist = $6,
            left_calf = $7,
            right_calf = $8,
            height = $9,
            neck = $10,
            hips = $11,
            torso = $12,
            waist = $13
        WHERE 
            user_id = $14 AND date_at = $15;
        ",
            to_optional_f32(body_measurement.left_arm),
            to_optional_f32(body_measurement.right_arm),
            to_optional_f32(body_measurement.left_thigh),
            to_optional_f32(body_measurement.right_thigh),
            to_optional_f32(body_measurement.left_wrist),
            to_optional_f32(body_measurement.right_wrist),
            to_optional_f32(body_measurement.left_calf),
            to_optional_f32(body_measurement.right_calf),
            to_optional_f32(body_measurement.height),
            to_optional_f32(body_measurement.neck),
            to_optional_f32(body_measurement.hips),
            to_optional_f32(body_measurement.torso),
            to_optional_f32(body_measurement.waist),
            Uuid::from(body_measurement.user_id),
            NaiveDate::from(body_measurement.date_at)
        )
        .execute(self.database.as_ref())
        .await?;

        Ok(())
    }
    pub async fn delete(
        &self,
        user_id: &user::Id,
        date_at: &PastNaiveDate,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM users_body_measurements_cm WHERE user_id = $1 AND date_at = $2;",
            Uuid::from(user_id),
            NaiveDate::from(date_at)
        )
        .execute(self.database.as_ref())
        .await?;

        Ok(())
    }

    pub async fn get(
        &self,
        user_id: user::Id,
        date_at: PastNaiveDate,
    ) -> Result<Option<BodyMeasurementsCm>, sqlx::Error> {
        let record = sqlx::query!(
            "SELECT * FROM users_body_measurements_cm WHERE user_id = $1 AND date_at = $2;",
            Uuid::from(&user_id),
            NaiveDate::from(&date_at)
        )
        .fetch_optional(self.database.as_ref())
        .await?;

        Ok(record.map(|record| {
            BodyMeasurementsCm::builder(user_id, date_at)
                .left_arm(to_opt_pos_f32(record.left_arm))
                .right_arm(to_opt_pos_f32(record.right_arm))
                .left_thigh(to_opt_pos_f32(record.left_thigh))
                .right_thigh(to_opt_pos_f32(record.right_thigh))
                .left_wrist(to_opt_pos_f32(record.left_wrist))
                .right_wrist(to_opt_pos_f32(record.right_wrist))
                .left_calf(to_opt_pos_f32(record.left_calf))
                .right_calf(to_opt_pos_f32(record.right_calf))
                .height(to_opt_pos_f32(record.height))
                .neck(to_opt_pos_f32(record.neck))
                .hips(to_opt_pos_f32(record.hips))
                .torso(to_opt_pos_f32(record.torso))
                .waist(to_opt_pos_f32(record.waist))
                .build()
        }))
    }

    pub async fn get_all(
        &self,
        user_id: &user::Id,
    ) -> Result<Vec<BodyMeasurementsCm>, sqlx::Error> {
        let records = sqlx::query!(
            "SELECT * FROM users_body_measurements_cm WHERE user_id = $1;",
            Uuid::from(user_id),
        )
        .fetch_all(self.database.as_ref())
        .await?;

        Ok(records
            .iter()
            .map(|record| {
                BodyMeasurementsCm::builder(
                    user_id.clone(),
                    PastNaiveDate::from_trusted(&record.date_at),
                )
                .left_arm(to_opt_pos_f32(record.left_arm))
                .right_arm(to_opt_pos_f32(record.right_arm))
                .left_thigh(to_opt_pos_f32(record.left_thigh))
                .right_thigh(to_opt_pos_f32(record.right_thigh))
                .left_wrist(to_opt_pos_f32(record.left_wrist))
                .right_wrist(to_opt_pos_f32(record.right_wrist))
                .left_calf(to_opt_pos_f32(record.left_calf))
                .right_calf(to_opt_pos_f32(record.right_calf))
                .height(to_opt_pos_f32(record.height))
                .neck(to_opt_pos_f32(record.neck))
                .hips(to_opt_pos_f32(record.hips))
                .torso(to_opt_pos_f32(record.torso))
                .waist(to_opt_pos_f32(record.waist))
                .build()
            })
            .collect())
    }
}
