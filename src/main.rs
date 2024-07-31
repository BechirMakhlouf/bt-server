use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use configuration::ApplicationSettings;
use env_logger::Env;
use sqlx::Postgres;
use std::net::TcpListener;

mod configuration;
mod models;
mod repositories;
mod types;

#[get("/health")]
async fn health_check_service() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let application_settings = ApplicationSettings::get_settings().unwrap();

    let db_pool = application_settings
        .database
        .get_db_pool::<Postgres>()
        .await
        .unwrap();

    let _body_measurements_repository = repositories::BodyMeasurementsRepository::new(&db_pool);
    let _user_repository = repositories::UserRepository::new(&db_pool);
    let _user_info_repository = repositories::UserInfoRepository::new(&db_pool);
    let _user_weight_log_repository = repositories::UserWeightRepository::new(&db_pool);
    let _user_body_fat_repository = repositories::UserBodyFatRepository::new(&db_pool);

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let addr = TcpListener::bind(format!("127.0.0.1:{}", application_settings.port))
        .unwrap()
        .local_addr()
        .unwrap();

    println!("port: {}", addr.port());

    HttpServer::new(|| {
        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(health_check_service)
    })
    .bind(addr)?
    .run()
    .await
}
