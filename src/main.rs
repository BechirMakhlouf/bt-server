use std::net::TcpListener;

use configuration::ApplicationSettings;
use sqlx::Postgres;
use tonic::transport::Server;
mod auth;
mod configuration;
mod models;
mod repositories;
mod services;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv().unwrap();

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

    Server::builder()
        .add_service(services::create_reflection_service())
        .add_service(services::create_user_service(db_pool.clone()))
        .add_service(services::create_test_service())
        .serve(addr)
        .await?;

    Ok(())
}
