use actix_session::Session;
use actix_web::{error, web, HttpResponse, Responder};

use crate::{services::AppState, ACCESS_TOKEN_NAME};

pub async fn delete(
    session: Session,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let user_id = crate::middleware::is_authenticated(&session, &app_state)?;

    match app_state.repositories.user_info.delete(&user_id).await {
        Ok(()) => Ok(HttpResponse::Ok()),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}
