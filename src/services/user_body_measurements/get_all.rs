use actix_session::Session;
use actix_web::{error, web, HttpResponse, Responder};

use crate::services::AppState;

pub async fn get_all(
    session: Session,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let user_id = crate::middleware::is_authenticated(&session, &app_state)?;

    match app_state
        .repositories
        .user_body_measurements
        .get_all(&user_id)
        .await
    {
        Ok(logs) => Ok(HttpResponse::Ok().json(logs)),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}
