use actix_session::Session;
use actix_web::{error, web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{models::body_fat::BodyFatDate, services::AppState, ACCESS_TOKEN_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameters {
    pub date: NaiveDate,
}

pub async fn get(
    session: Session,
    query_params: web::Query<Parameters>,
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

    let body_fat_date = match BodyFatDate::new(query_params.date) {
        Ok(body_fat_date) => body_fat_date,
        Err(err) => return Err(error::ErrorBadRequest(err)),
    };

    match app_state
        .repositories
        .user_body_fat
        .get_user_log_by_date(&user_id, &body_fat_date)
        .await
    {
        Ok(Some(settings)) => Ok(HttpResponse::Ok().json(settings)),
        Ok(None) => Err(error::ErrorNotFound("User bodyfat not found.")),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
