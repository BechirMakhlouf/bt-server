use std::sync::Arc;

use chrono::NaiveDate;
use sqlx::{Pool, Postgres};

use crate::models::{
    user,
    user_weight::{UserWeight, WeightDate},
};

#[derive(Debug, Clone)]
pub struct UserWeightRepository {
    database: Arc<Pool<Postgres>>,
}

impl UserWeightRepository {
    pub fn new(db_pool: Arc<Pool<Postgres>>) -> Self {
        Self { database: db_pool }
    }

    pub async fn add_or_update(&self, weight_log: &UserWeight) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO users_weight (user_id, weight_kg, date_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, date_at)
            DO UPDATE SET
                weight_kg = EXCLUDED.weight_kg,
                date_at = EXCLUDED.date_at;
            ",
            weight_log.user_id.get_uuid(),
            f32::from(&weight_log.weight_kg),
            NaiveDate::from(&weight_log.date),
        )
        .execute(self.database.as_ref())
        .await?;

        Ok(())
    }

    pub async fn get_all_user_logs(
        &self,
        user_id: &user::Id,
    ) -> Result<Vec<UserWeight>, sqlx::Error> {
        //TODO: what to do when there are no logs? currently in produces an error

        Ok(sqlx::query!(
            "SELECT * FROM users_weight WHERE user_id = $1",
            user_id.get_uuid()
        )
        .fetch_all(self.database.as_ref())
        .await?
        .iter()
        .map(|weight_log_row| {
            UserWeight::new(
                weight_log_row.user_id.into(),
                weight_log_row.weight_kg,
                weight_log_row.date_at,
            )
            .unwrap()
        })
        .collect())
    }

    pub async fn get_user_logs_between_interval(
        &self,
        user_id: user::Id,
        start_date: WeightDate,
        end_date: WeightDate,
    ) -> Result<Vec<UserWeight>, sqlx::Error> {
        //TODO: what to do when there are no logs? currently in produces an error
        Ok(sqlx::query!(
            "SELECT * FROM users_weight WHERE user_id = $1 AND date_at BETWEEN $2 AND $3",
            user_id.get_uuid(),
            NaiveDate::from(start_date),
            NaiveDate::from(end_date)
        )
        .fetch_all(self.database.as_ref())
        .await?
        .iter()
        .map(|weight_log_row| {
            UserWeight::from_trusted(
                weight_log_row.user_id.into(),
                weight_log_row.weight_kg,
                weight_log_row.date_at,
            )
        })
        .collect())
    }

    pub async fn get_user_log_by_date(
        &self,
        user_id: &user::Id,
        date_at: &WeightDate,
    ) -> Result<Option<UserWeight>, sqlx::Error> {
        //TODO: what to do when there are no logs? currently in produces an error

        Ok(sqlx::query!(
            "SELECT * FROM users_weight WHERE user_id = $1 AND date_at = $2",
            user_id.get_uuid(),
            NaiveDate::from(date_at),
        )
        .fetch_optional(self.database.as_ref())
        .await?
        .map(|record| {
            UserWeight::from_trusted(record.user_id.into(), record.weight_kg, record.date_at)
        }))
    }

    pub async fn delete(
        &self,
        user_id: &user::Id,
        date_at: &WeightDate,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM users_weight WHERE date_at = $1 AND user_id = $2",
            NaiveDate::from(date_at),
            uuid::Uuid::from(user_id)
        )
        .execute(self.database.as_ref())
        .await?;

        Ok(())
    }
}
