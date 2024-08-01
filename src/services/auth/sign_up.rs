use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::span::Id;

use crate::{
    models::user::NewUser,
    services::{AppState, ResponseError},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpRequestBody {
    pub email: String,
    pub password: String,
}

pub async fn sign_up(
    body: web::Json<SignUpRequestBody>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let SignUpRequestBody { password, email } = body.into_inner();

    let new_user = match NewUser::new(&email, &password) {
        Ok(user) => user,
        Err(error) => return HttpResponse::BadRequest().json(ResponseError::new(&error)),
    };

    let user_repo = &app_state.into_inner().repositories.user_repository;

    let error = match user_repo.add(&new_user).await {
        Ok(_) => return HttpResponse::Ok().into(),
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
