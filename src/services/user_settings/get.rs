use actix_session::Session;
use actix_web::{error, web, HttpResponse, Responder};

use crate::{services::AppState, ACCESS_TOKEN_NAME};

pub async fn get(
    session: Session,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let access_token = match session.get::<String>(ACCESS_TOKEN_NAME) {
        Ok(Some(token)) => token,
        Ok(None) => return Err(error::ErrorUnauthorized("unauthenticated")),
        Err(err) => return Err(error::ErrorUnauthorized(err)),
    };

    let token_data = match app_state.session_factory.parse_session_jwt(&access_token) {
        Ok(token_data) => token_data,
        Err(err) => return Err(error::ErrorUnauthorized(err)),
    };

    let user_id = token_data.claims.user_id;

    match app_state.repositories.user_settings.get(&user_id).await {
        Ok(Some(settings)) => Ok(HttpResponse::Ok().json(settings)),
        Ok(None) => Err(error::ErrorNotFound("User settings not found.")),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
