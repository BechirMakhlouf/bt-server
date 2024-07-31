use sqlx::{Pool, Postgres};

use crate::models::{
    user,
    user_settings::{ThemeMode, Unit, UsersSettings},
};

use uuid::Uuid;

#[derive(Debug)]
pub struct UserSettingsRepository<'a> {
    database: &'a Pool<Postgres>,
}

impl<'a> UserSettingsRepository<'a> {
    pub fn new(db_pool: &'a Pool<Postgres>) -> Self {
        Self { database: db_pool }
    }

    pub async fn add(&self, user_settings: UsersSettings) -> Result<(), sqlx::Error> {
        let query = sqlx::query!(
            "insert into users_settings (user_id, pref_theme, pref_unit) values ($1, $2, $3)",
            Uuid::from(&user_settings.user_id),
            user_settings.pref_theme as ThemeMode,
            user_settings.pref_unit as Unit
        )
        .execute(self.database)
        .await?;

        Ok(())
    }

    pub async fn update(&self, user_settings: UsersSettings) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users_settings
             SET
                 pref_theme = $1, pref_unit = $2
            WHERE
                user_id = $3",
            user_settings.pref_theme as ThemeMode,
            user_settings.pref_unit as Unit,
            Uuid::from(&user_settings.user_id)
        )
        .execute(self.database)
        .await?;
        Ok(())
    }

    pub async fn get(&self, user_id: user::Id) -> Result<UsersSettings, sqlx::Error> {
        let query_result = sqlx::query!(
            "SELECT pref_unit as \"pref_unit: Unit\", pref_theme as \"pref_theme: ThemeMode\" FROM users_settings where user_id = $1",
            Uuid::from(&user_id)
        )
        .fetch_one(self.database)
        .await?;

        Ok(UsersSettings::new(
            user_id,
            query_result.pref_theme,
            query_result.pref_unit,
        ))
    }

    pub async fn delete(&self, user_settings: UsersSettings) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM users_settings where user_id = $1",
            Uuid::from(&user_settings.user_id)
        )
        .execute(self.database)
        .await?;
        Ok(())
    }
}
