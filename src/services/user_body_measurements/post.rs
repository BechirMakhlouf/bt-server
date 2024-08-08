use actix_session::Session;
use actix_web::{error, web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    models,
    services::AppState,
    types::{past_naive_date::PastNaiveDate, positive_non_zero_float::to_opt_pos_f32},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub date: NaiveDate,
    pub height: Option<f32>,
    pub left_arm: Option<f32>,
    pub right_arm: Option<f32>,
    pub left_thigh: Option<f32>,
    pub right_thigh: Option<f32>,
    pub left_wrist: Option<f32>,
    pub right_wrist: Option<f32>,
    pub neck: Option<f32>,
    pub left_calf: Option<f32>,
    pub right_calf: Option<f32>,
    pub hips: Option<f32>,
    pub torso: Option<f32>,
    pub waist: Option<f32>,
    pub weight_kg: Option<f32>,
}

pub async fn post(
    session: Session,
    query_params: web::Query<Parameters>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let user_id = crate::middleware::is_authenticated(&session, &app_state)?;

    let measurement_date = match PastNaiveDate::try_from(query_params.date) {
        Ok(date) => date,
        Err(err) => return Err(error::ErrorBadRequest(err)),
    };

    let body_measurements =
        models::body_measurements::BodyMeasurementsCm::builder(user_id, measurement_date)
            .left_arm(to_opt_pos_f32(query_params.left_arm))
            .right_arm(to_opt_pos_f32(query_params.right_arm))
            .left_thigh(to_opt_pos_f32(query_params.left_thigh))
            .right_thigh(to_opt_pos_f32(query_params.right_thigh))
            .left_wrist(to_opt_pos_f32(query_params.left_wrist))
            .right_wrist(to_opt_pos_f32(query_params.right_wrist))
            .left_calf(to_opt_pos_f32(query_params.left_calf))
            .right_calf(to_opt_pos_f32(query_params.right_calf))
            .height(to_opt_pos_f32(query_params.height))
            .neck(to_opt_pos_f32(query_params.neck))
            .hips(to_opt_pos_f32(query_params.hips))
            .torso(to_opt_pos_f32(query_params.torso))
            .waist(to_opt_pos_f32(query_params.waist))
            .build();

    match app_state
        .repositories
        .user_body_measurements
        .add(body_measurements)
        .await
    {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(sqlx::Error::Database(err)) => {
            if err.is_check_violation() {
                Err(error::ErrorBadRequest(err))
            } else if err.is_unique_violation() {
                Err(error::ErrorConflict(err))
            } else {
                Err(error::ErrorInternalServerError(err))
            }
        }
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}
