pub mod auth;
pub mod configuration;
pub mod guards;
pub mod models;
pub mod repositories;
pub mod services;
pub mod types;

use actix_web::{
    web::{scope, Data},
    App, HttpServer,
};
use configuration::ServerSettings;
use log::info;
use models::session::SessionFactory;
use repositories::Repositories;
use services::AppState;
use sqlx::PgPool;
use std::{net::TcpListener, sync::Arc};

const ACCESS_TOKEN_NAME: &str = "access-token";
const _REFRESH_TOKEN_NAME: &str = "refresh-token";

pub fn configure_app_state(settings: &ServerSettings) -> AppState {
    let db_pool = Arc::new(
        PgPool::connect_lazy(settings.database.database_url.as_str())
            .expect("failed to connect to postgres database"),
    );
    let session_factory = SessionFactory::new(settings.jwt_secret.clone(), "users".into(), 600);
    AppState::new(Repositories::new(db_pool), session_factory)
}

pub async fn run_server(settings: ServerSettings) -> std::io::Result<()> {
    let addr = TcpListener::bind(format!("{}:{}", settings.host, settings.port))
        .unwrap()
        .local_addr()
        .unwrap();

    info!("Server running on: {}", addr.to_string());

    let app_state = configure_app_state(&settings);

    HttpServer::new(move || {
        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .app_data(Data::new(app_state.clone()))
            .service(services::health::check)
            .service(services::auth::get_scope("/auth"))
            .service(
                scope("/user")
                    .service(services::user_profile::get_scope("/profile"))
                    .service(services::user_info::get_scope("/info"))
                    .service(services::user_settings::get_scope("/settings"))
                    .service(services::user_weight::get_scope("/weight"))
                    .service(services::user_body_fat::get_scope("/body-fat"))
                    .service(services::user_body_measurements::get_scope(
                        "/body-measurements",
                    )),
            )
    })
    .bind(addr)?
    .run()
    .await
}
