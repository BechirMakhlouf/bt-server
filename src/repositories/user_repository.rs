// all crud operations for user

use std::sync::Arc;

use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserRepository {
    database: Arc<Pool<Postgres>>,
}

use crate::models::user::{Email, EncryptedPassword, Id, NewUser, Password, User};

impl UserRepository {
    pub fn new(db_pool: Arc<Pool<Postgres>>) -> Self {
        Self { database: db_pool }
    }

    pub async fn add(&self, new_user: &NewUser) -> Result<Id, sqlx::Error> {
        let encrypted_password = new_user.password.hash_with_salt();

        let query_result = sqlx::query!(
            "INSERT INTO auth.users (email, encrypted_password) VALUES ($1, $2) RETURNING id",
            &new_user.email.as_str(),
            encrypted_password.to_string(),
        )
        .fetch_one(self.database.as_ref())
        .await?;

        Ok(query_result.id.into())
    }

    pub async fn get_by_id(&self, id: Id) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT * FROM auth.users WHERE id = $1",
            uuid::Uuid::from(id)
        )
        .fetch_one(self.database.as_ref())
        .await?;

        Ok(Some(User {
            id: result.id.into(),
            email: Email::from_trusted_str(&result.email),
            encrypted_password: EncryptedPassword::from_trusted_str(&result.encrypted_password),
        }))
    }

    pub async fn get_by_email(&self, email: &Email) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query!("SELECT * FROM auth.users WHERE email = $1", email.as_str())
            .fetch_optional(self.database.as_ref())
            .await?;

        match result {
            Some(result) => Ok(Some(User {
                id: result.id.into(),
                email: Email::from_trusted_str(&result.email),
                encrypted_password: EncryptedPassword::from_trusted_str(&result.encrypted_password),
            })),
            None => Ok(None),
        }
    }

    pub async fn delete(&self, id: &Id) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM auth.users WHERE id = $1", uuid::Uuid::from(id))
            .execute(self.database.as_ref())
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn update(&self, user: User) -> Result<User, sqlx::Error> {
        let uuid = Uuid::from(&user.id);

        let result = sqlx::query!(
            "UPDATE auth.users SET email = $1 WHERE id = $2 RETURNING *",
            user.email.as_str(),
            uuid,
        )
        .fetch_one(self.database.as_ref())
        .await?;

        Ok(User {
            id: result.id.into(),
            email: Email::from_trusted_str(&result.email),
            encrypted_password: EncryptedPassword::from_trusted_str(&result.encrypted_password),
        })
    }

    pub async fn update_password(
        &self,
        user: User,
        new_password: &Password,
    ) -> Result<User, sqlx::Error> {
        let uuid = Uuid::from(&user.id);

        let encrypted_password = new_password.hash_with_salt();

        let result = sqlx::query!(
            "UPDATE auth.users SET encrypted_password = $1 WHERE id = $2 returning *",
            encrypted_password.to_string(),
            uuid,
        )
        .fetch_one(self.database.as_ref())
        .await?;

        Ok(User {
            id: result.id.into(),
            email: Email::from_trusted_str(&result.email),
            encrypted_password: EncryptedPassword::from_trusted_str(&result.encrypted_password),
        })
    }
}
