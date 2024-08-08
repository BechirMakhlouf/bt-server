use std::sync::Arc;

use sqlx::{Pool, Postgres};

mod user;
mod user_body_fat;
mod user_body_measurements;
mod user_info;
mod user_profile;
mod user_settings;
mod user_weight;

pub use user::*;
pub use user_body_fat::*;
pub use user_body_measurements::*;
pub use user_info::*;
pub use user_profile::*;
pub use user_settings::*;
pub use user_weight::*;

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
