use sqlx::{Pool, Postgres};

use uuid::Uuid;

use crate::models::{user, user_profile::UserProfile};

pub struct UserProfileRepository<'a> {
    database: &'a Pool<Postgres>,
}

impl<'a> UserProfileRepository<'a> {
    pub fn new(db_pool: &'a Pool<Postgres>) -> Self {
        Self { database: db_pool }
    }

    pub async fn add(&self, user_profile: UserProfile) -> Result<(), sqlx::Error> {
        sqlx::query!("INSERT INTO users_profiles (user_id, url, picture_url, description) VALUES ($1, $2, $3, $4)",
        Uuid::from(&user_profile.user_id),
        user_profile.url.as_str(),
        user_profile.picture_url.as_str(),
        user_profile.description).execute(self.database).await?;

        Ok(())
    }
    pub async fn update(&self, user_profile: UserProfile) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users_profiles SET
                    url = $1,
                    picture_url = $2,
                    description = $3
            WHERE
                    user_id = $4",
            user_profile.url.as_str(),
            user_profile.picture_url.as_str(),
            user_profile.description,
            Uuid::from(user_profile.user_id)
        )
        .execute(self.database)
        .await?;

        Ok(())
    }

    pub async fn get(&self, user_id: &user::Id) -> Result<UserProfile, sqlx::Error> {
        let query_result = sqlx::query!(
            "SELECT * FROM users_profiles where user_id = $1",
            Uuid::from(user_id)
        )
        .fetch_one(self.database)
        .await?;
        Ok(UserProfile::new(
            query_result.user_id.into(),
            query_result.url.as_str().try_into().unwrap(),
            query_result.picture_url.as_str().try_into().unwrap(),
            query_result.description,
        ))
    }

    pub async fn delete(&self, user_id: &user::Id) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM users_profiles WHERE user_id = $1",
            Uuid::from(user_id)
        )
        .execute(self.database)
        .await?;

        Ok(())
    }
}
