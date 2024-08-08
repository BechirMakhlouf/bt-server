use actix_session::Session;
use actix_web::{error, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{models::user_settings::UsersSettings, services::AppState, ACCESS_TOKEN_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    pref_theme: String,
    pref_unit: String,
}

pub async fn put(
    session: Session,
    body: web::Json<RequestBody>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let user_id = crate::middleware::is_authenticated(&session, &app_state)?;

    let user_settings = match UsersSettings::try_from_strs(
        &user_id.to_string(),
        body.pref_theme.as_str(),
        body.pref_unit.as_str(),
    ) {
        Ok(user_settings) => user_settings,
        Err(err) => return Err(actix_web::error::ErrorBadRequest(err)),
    };

    match app_state
        .repositories
        .user_settings
        .update(&user_settings)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
    }
}
