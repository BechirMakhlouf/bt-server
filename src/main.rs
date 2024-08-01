use actix_web::{
    web::{scope, Data},
    App, HttpServer,
};
use configuration::ApplicationSettings;
use env_logger::Env;
use log::info;
use models::user_settings;
use repositories::Repositories;
use services::AppState;
use std::{net::TcpListener, sync::Arc};

mod auth;
mod configuration;
mod models;
mod repositories;
mod services;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let application_settings = ApplicationSettings::get_settings().unwrap();

    let db_pool = Arc::new(
        sqlx::postgres::PgPool::connect_lazy(application_settings.database.database_url.as_str())
            .expect("failed to connect to postgres database"),
    );

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let addr = TcpListener::bind(format!(
        "{}:{}",
        application_settings.host, application_settings.port
    ))
    .unwrap()
    .local_addr()
    .unwrap();

    info!("Server running on: {}", addr.to_string());

    let app_state = AppState::new(Repositories::new(db_pool));

    use services::*;
    let _ = HttpServer::new(move || {
        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .app_data(Data::new(app_state.clone()))
            .service(services::health::check)
            .service(services::auth::get_scope("/auth"))
            .service(
                scope("/user")
                    .service(user_profile::get_scope("/profile"))
                    .service(user_info::get_scope("/info"))
                    .service(user_settings::get_scope("/settings"))
                    .service(user_weight::get_scope("/weight"))
                    .service(user_body_fat::get_scope("/body-fat"))
                    .service(user_body_measurements::get_scope("/body-measurements")),
            )
    })
    .bind(addr)?
    .run()
    .await;

    info!("Server closing.: {}", addr.to_string());
    Ok(())
}
