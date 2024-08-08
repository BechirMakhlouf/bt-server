use actix_session::Session;
use actix_web::{error, web, HttpResponse, Responder};

use crate::{services::AppState, ACCESS_TOKEN_NAME};

pub async fn get(
    session: Session,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let user_id = crate::middleware::is_authenticated(&session, &app_state)?;

    match app_state.repositories.user_settings.get(&user_id).await {
        Ok(Some(settings)) => Ok(HttpResponse::Ok().json(settings)),
        Ok(None) => Err(error::ErrorNotFound("User settings not found.")),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
