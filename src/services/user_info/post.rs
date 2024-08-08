use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{middleware::is_authenticated, models::user_info::UserInfo, services::AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    pub username: String,
    pub gender: String,
    pub birthday: String,
}

pub async fn post(
    session: Session,
    body: web::Json<RequestBody>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let user_id = is_authenticated(&session, &app_state)?;

    let user_info = match UserInfo::try_from_strs(
        &String::from(user_id),
        &body.username,
        &body.gender,
        &body.birthday,
    ) {
        Ok(user_info) => user_info,
        Err(err) => return Err(actix_web::error::ErrorBadRequest(err)),
    };

    match app_state.repositories.user_info.add(user_info).await {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(sqlx::Error::Database(err)) => {
            if err.is_unique_violation() {
                Err(actix_web::error::ErrorConflict(err))
            } else {
                Err(actix_web::error::ErrorInternalServerError(err))
            }
        }
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
    }
}
