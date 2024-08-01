#![allow(unused)]
use std::sync::Arc;

use sqlx::{Pool, Postgres};

mod user_body_fat_repository;
mod user_body_measurements_repository;
mod user_info_repository;
mod user_profile_repository;
mod user_repository;
mod user_settings_repository;
mod user_weight_repository;

pub use user_body_fat_repository::*;
pub use user_body_measurements_repository::*;
pub use user_info_repository::*;
pub use user_profile_repository::*;
pub use user_repository::*;
pub use user_settings_repository::*;
pub use user_weight_repository::*;

#[derive(Debug, Clone)]
pub struct Repositories {
    pub user_body_fat_repository: UserBodyFatRepository,
    pub user_body_measurements_repository: UserBodyMeasurementsRepository,
    pub user_info_repository: UserInfoRepository,
    pub user_profile_repository: UserProfileRepository,
    pub user_repository: UserRepository,
    pub user_settings_repository: UserSettingsRepository,
    pub user_weight_repository: UserWeightRepository,
}

impl Repositories {
    pub fn new(db_pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            user_body_measurements_repository: UserBodyMeasurementsRepository::new(db_pool.clone()),
            user_body_fat_repository: UserBodyFatRepository::new(db_pool.clone()),
            user_info_repository: UserInfoRepository::new(db_pool.clone()),
            user_profile_repository: UserProfileRepository::new(db_pool.clone()),
            user_repository: UserRepository::new(db_pool.clone()),
            user_settings_repository: UserSettingsRepository::new(db_pool.clone()),
            user_weight_repository: UserWeightRepository::new(db_pool.clone()),
        }
    }
}
