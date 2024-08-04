use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        session::{self},
        user::UserCredentials,
    },
    services::{AppState, ResponseError},
    ACCESS_TOKEN_NAME,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInRequestBody {
    pub email: String,
    pub password: String,
}

pub async fn sign_in(
    body: web::Json<SignInRequestBody>,
    app_state: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    let SignInRequestBody { password, email } = body.into_inner();
    let app_state = app_state.into_inner();

    let user_credentials = match UserCredentials::new(&email, &password) {
        Ok(user) => user,
        Err(error) => return HttpResponse::BadRequest().json(ResponseError::new(&error)),
    };

    let user_repo = &app_state.repositories.user;

    let opt_user = match user_repo.get_by_email(&user_credentials.email).await {
        Ok(opt_user) => opt_user,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseError::new("internal error."))
        }
    };

    match opt_user {
        Some(user) => {
            let session_factory = &app_state.session_factory;
            let jwt = match session_factory.create_jwt_from_user_id(user.id) {
                Ok(jwt) => jwt,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };

            match session.insert(ACCESS_TOKEN_NAME, jwt) {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        None => HttpResponse::Unauthorized().json(ResponseError::new("Invalid Credentials")),
    }
}
