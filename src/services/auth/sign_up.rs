use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    models::user::UserCredentials,
    services::{AppState, ResponseError},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    pub email: String,
    pub password: String,
}

pub async fn sign_up(
    body: web::Json<RequestBody>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let RequestBody { password, email } = body.into_inner();

    let user_credentials = match UserCredentials::new(&email, &password) {
        Ok(user) => user,
        Err(error) => return HttpResponse::BadRequest().json(ResponseError::new(&error)),
    };

    let user_repo = &app_state.into_inner().repositories.user;

    let error = match user_repo.add(&user_credentials).await {
        Ok(_) => return HttpResponse::Created().into(),
        Err(err) => err,
    };

    match error {
        sqlx::Error::Database(err) => {
            if err.is_unique_violation() {
                HttpResponse::Conflict().json(ResponseError::new(err.message()))
            } else {
                HttpResponse::InternalServerError().json(ResponseError::new(err.message()))
            }
        }
        _ => HttpResponse::InternalServerError().json(ResponseError::new("Internal error.")),
    }
}
