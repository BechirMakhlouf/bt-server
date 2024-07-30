use configuration::ApplicationSettings;
use repositories::{
    BodyMeasurementsRepository, Repository, UserInfoRepository, UserRepository,
    UserWeightLogRepository,
};
use sqlx::Postgres;
use std::net::TcpListener;

mod auth;
mod configuration;
mod models;
mod repositories;
mod services;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = TcpListener::bind("[::1]:50051")
        .unwrap()
        .local_addr()
        .unwrap();

    println!("port: {}", addr.port());

    let application_settings = ApplicationSettings::get_settings().unwrap();

    let db_pool = application_settings
        .database
        .get_db_pool::<Postgres>()
        .await
        .unwrap();

    let body_measurements_repository = BodyMeasurementsRepository::new(&db_pool);
    let user_repository = UserRepository::new(&db_pool);
    let user_info_repository = UserInfoRepository::new(&db_pool);
    let user_weight_log_repository = UserWeightLogRepository::new(&db_pool);

    let _repo = Repository::new(
        body_measurements_repository,
        user_info_repository,
        user_weight_log_repository,
        user_repository,
    );

    // Server::builder()
    //     .add_service(services::create_reflection_service())
    //     .add_service(services::create_health_service())
    //     .add_service(services::create_user_service(
    //         db_pool.clone(),
    //         application_settings.jwt_secret,
    //     ))
    //     .add_service(services::create_test_service())
    //     .serve(addr)
    //     .await?;

    Ok(())
}
