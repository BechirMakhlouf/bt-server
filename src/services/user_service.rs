use crate::{
    models::NewUser,
    services::proto::{user_server::User, CreateUserRequest, CreateUserResponse},
};
use tonic::{Request, Response};

#[derive(Debug, Default)]
pub struct UserService {}

#[tonic::async_trait]
impl User for UserService {
    async fn create_user(
        &self,
        req: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, tonic::Status> {
        let user = req.get_ref();

        match NewUser::new(&user.email, &user.password) {
            Ok(new_user) => println!("new user created:  {new_user:?}"),
            Err(_) => print!("hello world"),
        }

        Ok(Response::new(CreateUserResponse::default()))
        // match NewUser::new(, password)
    }
}
