mod test_service;

pub mod proto {
    tonic::include_proto!("test");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("test_descriptor");
}

pub use proto::test_server::TestServer;
pub use test_service::TestService;
use tonic_health::pb::health_server::{Health, HealthServer};
use tonic_reflection::server::{ServerReflection, ServerReflectionServer};

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
