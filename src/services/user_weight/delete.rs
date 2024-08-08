use actix_session::Session;
use actix_web::{
    error,
    web::{self, Query},
    HttpResponse, Responder,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{models::user_weight::WeightDate, services::AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameters {
    pub date: NaiveDate,
}

pub async fn delete(
    session: Session,
    query_params: Query<Parameters>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let user_id = crate::middleware::is_authenticated(&session, &app_state)?;

    let weight_date = match WeightDate::new(query_params.date) {
        Ok(weight_date) => weight_date,
        Err(err) => return Err(error::ErrorBadRequest(err)),
    };

    match app_state
        .repositories
        .user_weight
        .delete(&user_id, &weight_date)
        .await
    {
        Ok(()) => Ok(HttpResponse::Ok()),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}
