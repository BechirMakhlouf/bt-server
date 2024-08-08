use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::models::{
    user,
    user_info::{self, Gender, UserInfo},
};
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserInfoRepository {
    database: Arc<Pool<Postgres>>,
}

impl<'a> UserInfoRepository {
    pub fn new(db_pool: Arc<Pool<Postgres>>) -> Self {
        Self { database: db_pool }
    }

    pub async fn add(&self, user_info: UserInfo) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users_info (user_id, username, gender, birthday) VALUES ($1, $2, $3, $4)",
            Uuid::from(user_info.user_id),
            String::from(user_info.username),
            user_info.gender as Gender,
            NaiveDate::from(user_info.birthday),
        )
        .execute(self.database.as_ref())
        .await?;

        Ok(())
    }
    pub async fn update(&self, user_info: UserInfo) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users_info
             SET
                 username = $1, gender = $2, birthday = $3
            WHERE
                user_id = $4",
            String::from(user_info.username),
            user_info.gender as Gender,
            NaiveDate::from(user_info.birthday),
            Uuid::from(user_info.user_id)
        )
        .execute(self.database.as_ref())
        .await?;
        Ok(())
    }

    pub async fn get(&self, user_id: &user::Id) -> Result<Option<UserInfo>, sqlx::Error> {
        let query_result = sqlx::query!(
            "SELECT user_id, username , gender AS \"gender: Gender\" , birthday
            FROM users_info WHERE user_id = $1",
            Uuid::from(user_id)
        )
        .fetch_optional(self.database.as_ref())
        .await?;

        match query_result {
            Some(result) => Ok(Some(
                user_info::UserInfo::new(
                    result.user_id.into(),
                    result.username,
                    result.gender,
                    result.birthday,
                )
                .unwrap(),
            )),
            None => Ok(None),
        }
    }
    pub async fn delete(&self, user_id: &user::Id) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM users_info WHERE user_id = $1",
            Uuid::from(user_id)
        )
        .execute(self.database.as_ref())
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::NaiveDate;

    use crate::{
        models::{
            user,
            user_info::{self, UserInfo},
        },
        repositories::UserRepository,
    };

    use super::UserInfoRepository;

    #[tokio::test]
    async fn test_add_get() {
        let application_settings = crate::Settings::get_settings().unwrap();

        let db_pool = Arc::new(application_settings.database.get_db_pool().unwrap());

        let user_repo = UserRepository::new(db_pool.clone());
        let user_info_repo = UserInfoRepository::new(db_pool.clone());

        let user_credentials =
            user::UserCredentials::new("emaile@gmail.com", "dlskfjsdlkfjd").unwrap();
        let user_id = user_repo.add(&user_credentials).await.unwrap();

        let user_info = UserInfo::new(
            user_id.clone(),
            "hello_world".into(),
            user_info::Gender::Other,
            NaiveDate::from_ymd_opt(2001, 10, 12).unwrap(),
        )
        .unwrap();

        user_info_repo
            .add(user_info.clone())
            .await
            .expect("user_info should be added without problems");

        let retreived_user_info = user_info_repo
            .get(&user_id)
            .await
            .expect("user_info should be retreived without errors.")
            .expect("retreived user should not be none");
        assert_eq!(retreived_user_info.user_id, user_info.user_id);

        let _ = user_info_repo.delete(&user_id).await;
    }
}
