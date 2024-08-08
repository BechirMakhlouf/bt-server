pub mod auth;
pub mod configuration;
pub mod file_storage;
pub mod guards;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;
pub mod types;

use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key, SameSite},
    middleware::Logger,
    web::{scope, Data},
    App, HttpServer,
};
use configuration::Settings;
use file_storage::AwsMediaStorage;
use log::info;
use models::session::SessionFactory;
use repositories::Repositories;
use services::AppState;
use std::{net::TcpListener, sync::Arc};

const ACCESS_TOKEN_NAME: &str = "access-token";
const _REFRESH_TOKEN_NAME: &str = "refresh-token";
const SECS_IN_WEEK: u64 = 604800;

pub async fn configure_app_state(settings: &Settings) -> AppState {
    let db_pool = Arc::new(
        settings
            .database
            .get_db_pool()
            .expect("failed to connect to postgres database"),
    );

    let session_factory = SessionFactory::new(settings.jwt_secret.clone(), "users".into(), 60000);

    let aws_shared_config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&aws_shared_config);

    let aws_media_storage = AwsMediaStorage::new(
        s3_client,
        "bodytrackr".into(),
        std::time::Duration::from_secs(SECS_IN_WEEK),
    );

    AppState::new(
        Repositories::new(db_pool),
        session_factory,
        settings.app_env.clone(),
        aws_media_storage,
    )
}

pub async fn run_server(settings: Settings) -> std::io::Result<()> {
    let addr = TcpListener::bind(format!("{}:{}", settings.host, settings.port))
        .unwrap()
        .local_addr()
        .unwrap();

    info!("Server running on: {}", addr.to_string());

    let app_state = configure_app_state(&settings).await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(app_state.clone()))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_name("session".into())
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(Duration::seconds(SECS_IN_WEEK.try_into().unwrap())),
                    )
                    .cookie_same_site(SameSite::Strict)
                    .build(),
            )
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
