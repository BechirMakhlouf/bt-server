use actix_session::Session;
use actix_web::{error, web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{services::AppState, types::past_naive_date::PastNaiveDate, ACCESS_TOKEN_NAME};

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

    let date_at = match PastNaiveDate::try_from(query_params.date) {
        Ok(date_at) => date_at,
        Err(err) => return Err(error::ErrorBadRequest(err)),
    };

    match app_state
        .repositories
        .user_body_measurements
        .get(user_id, date_at)
        .await
    {
        Ok(Some(settings)) => Ok(HttpResponse::Ok().json(settings)),
        Ok(None) => Err(error::ErrorNotFound("User weight not found.")),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
