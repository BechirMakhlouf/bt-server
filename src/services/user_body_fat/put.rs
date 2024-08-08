use actix_session::Session;
use actix_web::{error, web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{models::body_fat::UserBodyFat, services::AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    pref_theme: String,
    pref_unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub body_fat: f32,
    pub date: NaiveDate,
}

pub async fn put(
    session: Session,
    body: web::Query<Parameters>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let user_id = crate::middleware::is_authenticated(&session, &app_state)?;

    let user_body_fat = match UserBodyFat::new(user_id, body.body_fat, body.date) {
        Ok(user_weight) => user_weight,
        Err(err) => return Err(error::ErrorBadRequest(err)),
    };

    match app_state
        .repositories
        .user_body_fat
        .add_or_update(&user_body_fat)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
    }
}
