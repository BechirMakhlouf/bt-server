use std::sync::Arc;

use chrono::NaiveDate;
use sqlx::{Pool, Postgres};

use crate::models::{body_fat::UserBodyFat, user, user_weight::WeightDate};

#[derive(Debug, Clone)]
pub struct UserBodyFatRepository {
    database: Arc<Pool<Postgres>>,
}

impl UserBodyFatRepository {
    pub fn new(db_pool: Arc<Pool<Postgres>>) -> Self {
        Self { database: db_pool }
    }

    pub async fn add_or_update(&self, users_body_fat: UserBodyFat) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO users_body_fat (user_id, body_fat, date_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (date_at, user_id)
            DO UPDATE SET
                body_fat = EXCLUDED.body_fat,
                date_at = EXCLUDED.date_at;
            ",
            users_body_fat.user_id.get_value(),
            f32::from(users_body_fat.body_fat),
            NaiveDate::from(users_body_fat.date),
        )
        .execute(self.database.as_ref())
        .await?;

        Ok(())
    }

    pub async fn get_all_user_logs(
        &self,
        user_id: user::Id,
    ) -> Result<Vec<UserBodyFat>, sqlx::Error> {
        //TODO: what to do when there are no logs? currently in produces an error

        Ok(sqlx::query!(
            "SELECT * FROM users_body_fat WHERE user_id = $1",
            user_id.get_value()
        )
        .fetch_all(self.database.as_ref())
        .await?
        .iter()
        .map(|user_body_fat_row| {
            UserBodyFat::new(
                user_body_fat_row.user_id.into(),
                user_body_fat_row.body_fat,
                user_body_fat_row.date_at,
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
    ) -> Result<Vec<UserBodyFat>, sqlx::Error> {
        //TODO: what to do when there are no logs? currently in produces an error

        Ok(sqlx::query!(
            "SELECT * FROM users_body_fat WHERE user_id = $1 AND date_at BETWEEN $2 AND $3",
            user_id.get_value(),
            NaiveDate::from(start_date),
            NaiveDate::from(end_date)
        )
        .fetch_all(self.database.as_ref())
        .await?
        .iter()
        .map(|user_body_fat_row| {
            UserBodyFat::new(
                user_body_fat_row.user_id.into(),
                user_body_fat_row.body_fat,
                user_body_fat_row.date_at,
            )
            .unwrap()
        })
        .collect())
    }

    pub async fn delete(&self, user_body_fat: UserBodyFat) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM users_body_fat WHERE date_at = $1 AND user_id = $2",
            NaiveDate::from(user_body_fat.date),
            uuid::Uuid::from(user_body_fat.user_id)
        )
        .execute(self.database.as_ref())
        .await?;

        Ok(())
    }
}
