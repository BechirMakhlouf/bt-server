// all crud operations for user

use sqlx::{Pool, Postgres};
use uuid::Uuid;

// trait UserRepositoryInterface {
//     async fn add(&self, new_user: &NewUser) -> Self;
//     async fn get_by_id(&self, id: Id) -> Result<Option<User>, sqlx::Error>;
//     async fn get_by_email(&self, id: Id) -> Result<Option<User>, sqlx::Error>;
//     async fn delete(&self, id: &Id) -> Result<u64, sqlx::Error>;
//     async fn update(&self, user: User) -> Result<User, sqlx::Error>;
//     async fn update_password(
//         &self,
//         user: User,
//         new_password: &Password,
//     ) -> Result<User, sqlx::Error>;
// }

#[derive(Debug)]
pub struct UserRepository {
    database: Pool<Postgres>,
}

use crate::models::user::{Email, HashedPassword, Id, NewUser, Password, User};

impl UserRepository {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { database: db_pool }
    }
    pub async fn add(&self, new_user: &NewUser) -> Result<Id, sqlx::Error> {
        let hashed_password = new_user.password.hash_with_salt();

        let query_result = sqlx::query!(
            "INSERT INTO users (email, hashed_password) VALUES ($1, $2) RETURNING id",
            &new_user.email.as_str(),
            hashed_password.to_string(),
        )
        .fetch_one(&self.database)
        .await?;

        Ok(Id::from(query_result.id))
    }

    pub async fn get_by_id(&self, id: Id) -> Result<Option<User>, sqlx::Error> {
        let uuid = Uuid::from(&id);
        let result = sqlx::query!("SELECT * FROM users WHERE id = $1", uuid)
            .fetch_one(&self.database)
            .await?;

        Ok(Some(User {
            id: Id::from(result.id),
            email: Email::from_trusted_str(&result.email),
            hashed_password: HashedPassword::from_trusted_str(&result.hashed_password),
        }))
    }

    pub async fn get_by_email(&self, email: &Email) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query!("SELECT * FROM users WHERE email = $1", email.as_str())
            .fetch_optional(&self.database)
            .await?;

        match result {
            Some(result) => Ok(Some(User {
                id: Id::from(result.id),
                email: Email::from_trusted_str(&result.email),
                hashed_password: HashedPassword::from_trusted_str(&result.hashed_password),
            })),
            None => Ok(None),
        }
    }

    pub async fn delete(&self, id: &Id) -> Result<u64, sqlx::Error> {
        let uuid: uuid::Uuid = Uuid::from(id);

        let result = sqlx::query!("DELETE FROM users WHERE id = $1", uuid)
            .execute(&self.database)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn update(&self, user: User) -> Result<User, sqlx::Error> {
        let uuid = Uuid::from(&user.id);

        let result = sqlx::query!(
            "UPDATE users SET email = $1 WHERE id = $2 RETURNING *",
            user.email.as_str(),
            uuid,
        )
        .fetch_one(&self.database)
        .await?;

        Ok(User {
            id: Id::from(result.id),
            email: Email::from_trusted_str(&result.email),
            hashed_password: HashedPassword::from_trusted_str(&result.hashed_password),
        })
    }

    pub async fn update_password(
        &self,
        user: User,
        new_password: &Password,
    ) -> Result<User, sqlx::Error> {
        let uuid = Uuid::from(&user.id);

        let hashed_password = new_password.hash_with_salt();

        let result = sqlx::query!(
            "UPDATE users SET hashed_password = $1 WHERE id = $2 returning *",
            hashed_password.to_string(),
            uuid,
        )
        .fetch_one(&self.database)
        .await?;

        Ok(User {
            id: Id::from(result.id),
            email: Email::from_trusted_str(&result.email),
            hashed_password: HashedPassword::from_trusted_str(&result.hashed_password),
        })
    }
}
