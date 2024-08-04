use actix_session::Session;
use actix_web::{error, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{models::user_info::UserInfo, services::AppState, ACCESS_TOKEN_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    pub username: String,
    pub gender: String,
    pub birthday: String,
}

pub async fn put(
    session: Session,
    body: web::Json<RequestBody>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let access_token = match session.get::<String>(ACCESS_TOKEN_NAME) {
        Ok(Some(token)) => token,
        Ok(None) => return Err(error::ErrorUnauthorized("Unauthenticated")),
        Err(err) => return Err(error::ErrorUnauthorized(err)),
    };

    let token_data = match app_state.session_factory.parse_session_jwt(&access_token) {
        Ok(token_data) => token_data,
        Err(err) => return Err(error::ErrorUnauthorized(err)),
    };

    let user_id = token_data.claims.user_id;

    let user_info = match UserInfo::try_from_strs(
        &String::from(user_id),
        &body.username,
        &body.gender,
        &body.birthday,
    ) {
        Ok(user_info) => user_info,
        Err(err) => return Err(actix_web::error::ErrorBadRequest(err)),
    };

    match app_state.repositories.user_info.update(user_info).await {
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
