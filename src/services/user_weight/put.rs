use actix_session::Session;
use actix_web::{error, web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{models::user_weight::UserWeight, services::AppState, ACCESS_TOKEN_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    pref_theme: String,
    pref_unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub weight_kg: f32,
    pub date: NaiveDate,
}

pub async fn put(
    session: Session,
    body: web::Query<Parameters>,
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

    let weight_log = match UserWeight::new(user_id, body.weight_kg, body.date) {
        Ok(user_weight) => user_weight,
        Err(err) => return Err(error::ErrorBadRequest(err)),
    };

    match app_state
        .repositories
        .user_weight
        .add_or_update(&weight_log)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
    }
}
