use actix_session::Session;
use actix_web::{error, web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{models::user_weight::UserWeight, services::AppState};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub weight_kg: f32,
    pub date: NaiveDate,
}

pub async fn put(
    session: Session,
    query_params: web::Query<Parameters>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let user_id = crate::middleware::is_authenticated(&session, &app_state)?;

    let weight_log = match UserWeight::new(user_id, query_params.weight_kg, query_params.date) {
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
