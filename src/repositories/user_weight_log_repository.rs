use chrono::NaiveDate;
use sqlx::{query, Pool, Postgres};
use tonic::IntoStreamingRequest;

use crate::models::{
    user,
    weight_log::{self, WeightDate, WeightLog},
};

#[derive(Debug)]
pub struct UserWeightLogRepository<'a> {
    database: &'a Pool<Postgres>,
}

impl<'a> UserWeightLogRepository<'a> {
    pub fn new(db_pool: &'a Pool<Postgres>) -> Self {
        Self { database: db_pool }
    }

    pub async fn add_or_update(&self, weight_log: WeightLog) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO users_weight_log (user_id, weight_kg, date_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (date_at, user_id)
            DO UPDATE SET
                weight_kg = EXCLUDED.weight_kg,
                date_at = EXCLUDED.date_at;
            ",
            weight_log.user_id.get_value(),
            f32::from(weight_log.weight_kg),
            NaiveDate::from(weight_log.date),
        )
        .execute(self.database)
        .await?;

        Ok(())
    }

    pub async fn get_all_user_logs(
        &self,
        user_id: user::Id,
    ) -> Result<Vec<WeightLog>, sqlx::Error> {
        //TODO: what to do when there are no logs? currently in produces an error

        Ok(sqlx::query!(
            "SELECT * FROM users_weight_log WHERE user_id = $1",
            user_id.get_value()
        )
        .fetch_all(self.database)
        .await?
        .iter()
        .map(|weight_log_row| {
            WeightLog::new(
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
    ) -> Result<Vec<WeightLog>, sqlx::Error> {
        //TODO: what to do when there are no logs? currently in produces an error

        Ok(sqlx::query!(
            "SELECT * FROM users_weight_log WHERE user_id = $1 AND date_at BETWEEN $2 AND $3",
            user_id.get_value(),
            NaiveDate::from(start_date),
            NaiveDate::from(end_date)
        )
        .fetch_all(self.database)
        .await?
        .iter()
        .map(|weight_log_row| {
            WeightLog::new(
                weight_log_row.user_id.into(),
                weight_log_row.weight_kg,
                weight_log_row.date_at,
            )
            .unwrap()
        })
        .collect())
    }

    pub async fn delete(&self, weight_log: WeightLog) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM users_weight_log WHERE date_at = $1 AND user_id = $2",
            NaiveDate::from(weight_log.date),
            uuid::Uuid::from(weight_log.user_id)
        )
        .execute(self.database)
        .await?;

        Ok(())
    }
}
