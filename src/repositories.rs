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
    pub user_body_fat: UserBodyFatRepository,
    pub user_body_measurements: UserBodyMeasurementsRepository,
    pub user_info: UserInfoRepository,
    pub user_profile: UserProfileRepository,
    pub user: UserRepository,
    pub user_settings: UserSettingsRepository,
    pub user_weight: UserWeightRepository,
}

impl Repositories {
    pub fn new(db_pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            user_body_measurements: UserBodyMeasurementsRepository::new(db_pool.clone()),
            user_body_fat: UserBodyFatRepository::new(db_pool.clone()),
            user_info: UserInfoRepository::new(db_pool.clone()),
            user_profile: UserProfileRepository::new(db_pool.clone()),
            user: UserRepository::new(db_pool.clone()),
            user_settings: UserSettingsRepository::new(db_pool.clone()),
            user_weight: UserWeightRepository::new(db_pool.clone()),
        }
    }
}
