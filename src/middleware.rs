use actix_session::Session;
use actix_web::{error, web};

use crate::{models::user, services::AppState, ACCESS_TOKEN_NAME};

pub fn is_authenticated(
    session: &Session,
    app_state: &web::Data<AppState>,
) -> Result<user::Id, actix_web::Error> {
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

    Ok(user_id)
}
