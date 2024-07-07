use super::proto::*;
use tonic::{Request, Response};

#[derive(Default)]
pub struct TestService {}

#[tonic::async_trait]
impl test_server::Test for TestService {
    async fn test(&self, _: Request<TestRequest>) -> Result<Response<TestResponse>, tonic::Status> {
        let test_response = TestResponse {
            message: "working".into(),
        };

        Ok(Response::new(test_response))
    }
}
