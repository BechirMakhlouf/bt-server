use actix_session::Session;
use actix_web::{HttpResponse, Responder};

pub async fn sign_out(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok().finish()
}
