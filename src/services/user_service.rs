use crate::{
    models::user::NewUser,
    repositories::UserRepository,
    services::proto::{user_server::User, CreateUserRequest, CreateUserResponse},
};
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

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

        match self.user_repo.add(&new_user).await {
            Ok(user_id) => Ok(Response::new(CreateUserResponse {
                id: user_id.to_string(),
            })),
            Err(e) => Err(Status::new(tonic::Code::Aborted, e.to_string())),
        }
    }
}
