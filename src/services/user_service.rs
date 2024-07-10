use crate::{
    models::user::NewUser,
    repositories::UserRepository,
    services::proto::{user_server::User, CreateUserRequest, CreateUserResponse},
};
use sqlx::{error::ErrorKind, Pool, Postgres};
use tonic::{Code, Request, Response, Status};

#[derive(Debug)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self {
            user_repo: UserRepository::new(db_pool),
        }
    }
}

#[tonic::async_trait]
impl User for UserService {
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

        let query_result = self.user_repo.add(&new_user).await;

        if let Ok(user_id) = query_result {
            return Ok(Response::new(CreateUserResponse {
                id: user_id.to_string(),
            }));
        }

        let query_result = query_result.unwrap_err();

        if let sqlx::Error::Database(err) = query_result {
            match err.kind() {
                ErrorKind::UniqueViolation => {
                    return { Err(Status::already_exists("Email already exists.")) }
                }
                _ => return Err(Status::unknown("Unknown problem.")),
            }
        }

        return Err(Status::internal("Internal error while persisting."));
    }
}
