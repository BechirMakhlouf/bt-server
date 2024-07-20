use crate::{
    auth::{self, Authenticator},
    models::user::NewUser,
    services::proto::{user_server::User, CreateUserRequest, CreateUserResponse},
};
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use super::proto::{AuthUserRequest, AuthUserResponse};

#[derive(Debug)]
pub struct UserService {
    auth: Authenticator,
}

impl UserService {
    pub fn new(db_pool: Pool<Postgres>, jwt_secret: secrecy::Secret<String>) -> Self {
        Self {
            auth: Authenticator::new(db_pool, jwt_secret),
        }
    }
}

#[tonic::async_trait]
impl User for UserService {
    async fn auth_user(
        &self,
        req: Request<AuthUserRequest>,
    ) -> Result<Response<AuthUserResponse>, tonic::Status> {
        let AuthUserRequest { email, password } = req.get_ref();

        let new_user = NewUser::new(email, password);
        if let Err(err) = new_user {
            return Err(Status::invalid_argument(err));
        }
        let new_user = new_user.unwrap();
        let auth_user_result = self
            .auth
            .auth_user_email_password(new_user.email, new_user.password)
            .await;

        if let Ok(session) = auth_user_result {
            return Ok(Response::new(AuthUserResponse {
                session_token: self.auth.create_session_token(session).unwrap(),
            }));
        }
        let error = auth_user_result.unwrap_err();

        Err(match error {
            auth::Error::WrongCredentials => Status::unauthenticated(error.to_string()),
            _ => Status::internal(error.to_string()),
        })
    }

    async fn create_user(
        &self,
        req: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, tonic::Status> {
        let CreateUserRequest { email, password } = req.get_ref();
        let new_user = NewUser::new(email, password);

        if let Err(err) = new_user {
            return Err(tonic::Status::new(tonic::Code::InvalidArgument, err));
        }

        let new_user = new_user.unwrap();

        let sign_up_result = self.auth.sign_up_user(new_user).await;

        if let Ok(user_id) = sign_up_result {
            return Ok(Response::new(CreateUserResponse {
                id: user_id.to_string(),
            }));
        }

        let sign_up_result = sign_up_result.unwrap_err();

        match sign_up_result {
            auth::Error::UsedEmail(email) => {
                return Err(Status::already_exists(format!(
                    "email: `{email}` already exists."
                )))
            }
            _ => return Err(Status::unknown("Encountered unknown problem.")),
        };
    }
}
