use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};

use crate::{services::AppState, ACCESS_TOKEN_NAME};

pub async fn is_authenticated(session: Session, app_state: web::Data<AppState>) -> impl Responder {
    let access_token = match session.get::<String>(ACCESS_TOKEN_NAME) {
        Ok(Some(token)) => token,
        _ => return HttpResponse::Unauthorized().finish(),
    };

    match &app_state.session_factory.parse_session_jwt(&access_token) {
        Err(_) => HttpResponse::Unauthorized().finish(),
        _ => HttpResponse::Ok().finish(),
    }
}
