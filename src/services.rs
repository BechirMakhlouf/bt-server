#![allow(unused)]
mod test_service;
mod user_service;
pub mod proto {
    tonic::include_proto!("test");
    tonic::include_proto!("user");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("descriptor");
}

pub use proto::test_server::TestServer;
use proto::user_server::UserServer;
pub use test_service::TestService;

use tonic_health::pb::health_server::{Health, HealthServer};
use tonic_reflection::server::{ServerReflection, ServerReflectionServer};
use user_service::UserService;

pub fn create_health_service() -> HealthServer<impl Health> {
    let (_, health_service) = tonic_health::server::health_reporter();
    health_service
}

pub fn create_reflection_service() -> ServerReflectionServer<impl ServerReflection> {
    tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap()
}

pub fn create_test_service() -> TestServer<TestService> {
    TestServer::new(TestService::default())
}

pub fn create_user_service() -> UserServer<UserService> {
    UserServer::new(UserService::default())
}
