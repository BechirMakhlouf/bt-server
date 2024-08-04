use env_logger::Env;
use server::configuration::Settings;
use server::run_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let settings = Settings::get_settings().unwrap();
    run_server(settings).await
}
