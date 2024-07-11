use jsonwebtoken::{Algorithm, Header, TokenData};
use serde::{Deserialize, Serialize};

use crate::models::session::{Session, SessionId};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SessionTokenClaims {
    iat: usize,
    exp: usize,
    aud: String,
    session_id: SessionId,
}

pub fn create_token(session: Session) -> Result<String, jsonwebtoken::errors::Error> {
    let session_token_claims = SessionTokenClaims {
        session_id: session.id.clone(),
        iat: chrono::Local::now().timestamp().try_into().unwrap(),
        exp: session.exp.timestamp().try_into().unwrap(),
        aud: "users".into(),
    };

    let header: Header = Header::new(Algorithm::HS384);
    let secret_key = b"my_secret_key";
    let encoded_token = jsonwebtoken::encode(
        &header,
        &session_token_claims,
        &jsonwebtoken::EncodingKey::from_secret(secret_key),
    );

    encoded_token
}

fn parse_jwt(value: &str) -> Result<TokenData<SessionTokenClaims>, jsonwebtoken::errors::Error> {
    let mut validator = jsonwebtoken::Validation::new(Algorithm::HS384);
    validator.set_audience(&["users"]);

    jsonwebtoken::decode::<SessionTokenClaims>(
        value,
        &jsonwebtoken::DecodingKey::from_secret(b"my_secret_key"),
        &validator,
    )
}

#[cfg(test)]
mod tests {

    use chrono::Days;

    use crate::auth::{create_token, parse_jwt};
    use crate::models::session::Session;
    use crate::models::user::UserId;

    #[test]
    fn try_tokenizing_sessions() {
        let session = Session::new(UserId::new(), Days::new(10));
        let jwt_str = create_token(session.clone()).expect("should return jwt string");
        let token = parse_jwt(&jwt_str).expect("should return session");

        assert_eq!(token.claims.session_id, session.id);
    }
}
