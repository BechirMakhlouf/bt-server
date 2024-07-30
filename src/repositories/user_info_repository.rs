use sqlx::{Pool, Postgres};

use crate::models::{
    user,
    user_info::{self, Birthday, Gender, UserInfo, Username},
};
use chrono::NaiveDate;
use uuid::Uuid;

pub struct UserInfoRepository<'a> {
    database: &'a Pool<Postgres>,
}

impl<'a> UserInfoRepository<'a> {
    pub fn new(db_pool: &'a Pool<Postgres>) -> Self {
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
        .execute(self.database)
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
        .execute(self.database)
        .await?;
        Ok(())
    }

    pub async fn get(&self, user_id: user::Id) -> Result<UserInfo, sqlx::Error> {
        let query_result = sqlx::query!(
            "SELECT user_id, username , gender AS \"gender: Gender\" , birthday
            FROM users_info WHERE user_id = $1",
            Uuid::from(user_id)
        )
        .fetch_one(self.database)
        .await?;

        Ok(user_info::UserInfo::new(
            query_result.user_id.into(),
            query_result.username.try_into().unwrap(),
            query_result.gender,
            query_result.birthday.try_into().unwrap(),
        )
        .unwrap())
    }
    pub async fn delete(&self, user_id: user::Id) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM users_info WHERE user_id = $1",
            Uuid::from(user_id)
        )
        .execute(self.database)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use sqlx::Postgres;

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
        let application_settings = crate::ApplicationSettings::get_settings().unwrap();

        let db_pool = application_settings
            .database
            .get_db_pool::<Postgres>()
            .await
            .unwrap();

        let user_repo = UserRepository::new(db_pool.clone());
        let user_info_repo = UserInfoRepository::new(&db_pool);

        let new_user = user::NewUser::new("emaile@gmail.com", "dlskfjsdlkfjd").unwrap();
        let user_id = user_repo.add(&new_user).await.unwrap();

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
            .get(user_id)
            .await
            .expect("user_info should be retreived without errors.");

        assert_eq!(retreived_user_info.user_id, user_info.user_id)
    }
}
