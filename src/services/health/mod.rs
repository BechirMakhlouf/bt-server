use actix_session::Session;
use actix_web::{get, HttpResponse, Responder};
use log::info;

#[get("/health")]
pub async fn check(session: Session) -> impl Responder {
    info!("health checked: {:?}", session.entries());
    HttpResponse::Ok()
}
