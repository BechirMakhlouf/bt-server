use actix_session::Session;
use actix_web::{error, web, HttpResponse, Responder};

use crate::services::AppState;

pub async fn get(
    session: Session,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let user_id = crate::middleware::is_authenticated(&session, &app_state)?;

    match app_state.repositories.user_info.get(&user_id).await {
        Ok(Some(info)) => Ok(HttpResponse::Ok().json(info)),
        Ok(None) => Err(error::ErrorNotFound("User info not found")),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
