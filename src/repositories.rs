#![allow(unused)]
use sqlx::{Pool, Postgres};
use tonic::{transport::Body, IntoRequest};

mod body_measurements_repository;
mod session_repository;
mod user_info_repository;
mod user_repository;
mod user_weight_log_repository;

pub use body_measurements_repository::*;
pub use session_repository::*;
pub use user_info_repository::*;
pub use user_repository::*;
pub use user_weight_log_repository::*;

pub struct Repository<'a> {
    body_measurements_repository: BodyMeasurementsRepository<'a>,
    user_info_repository: UserInfoRepository<'a>,
    user_weight_log_repository: UserWeightLogRepository<'a>,
    user_repository: UserRepository<'a>,
}

impl<'a> Repository<'a> {
    pub fn new(
        body_measurements_repository: BodyMeasurementsRepository<'a>,
        user_info_repository: UserInfoRepository<'a>,
        user_weight_log_repository: UserWeightLogRepository<'a>,
        user_repository: UserRepository<'a>,
    ) -> Self {
        return Self {
            body_measurements_repository,
            user_info_repository,
            user_weight_log_repository,
            user_repository,
        };
    }
}
