#![allow(unused)]

use std::{future::Future, path};

use aws_sdk_s3::{
    error::SdkError, operation::head_object::HeadObjectError, presigning::PresigningConfig,
    types::ObjectCannedAcl,
};
use serde::Serialize;

#[derive(Debug, Serialize, thiserror::Error)]
pub enum Error {
    #[error("Failed to upload file: {0}")]
    UploadFileFailed(String),

    #[error("Failed to get file url: {0}")]
    FileUrlRetreivalFailed(String),

    #[error("Internal Error: {0}")]
    InternalError(String),

    #[error("Requested file not found: {0}")]
    FileNotFound(String),
}

type Result<T> = std::result::Result<T, Error>;

pub trait MediaStorage {
    fn put(
        &self,
        file_path: &path::Path,
        media_id: &str,
        content_type: &str,
        is_private: bool,
    ) -> impl Future<Output = Result<()>>;
    fn get_url(&self, media_id: &str) -> impl Future<Output = Result<String>>;
    fn delete(&self, media_id: &str) -> impl Future<Output = Result<()>>;
}

pub struct AwsMediaStorage {
    bucket_name: String,
    client: aws_sdk_s3::Client,
    url_duration: std::time::Duration,
}

impl AwsMediaStorage {
    pub fn new(
        client: aws_sdk_s3::Client,
        bucket_name: String,
        url_duration: std::time::Duration,
    ) -> Self {
        Self {
            client,
            bucket_name,
            url_duration,
        }
    }
    async fn check_file_exists(&self, object_key: &str) -> Result<bool> {
        let result = self
            .client
            .head_object()
            .bucket(&self.bucket_name)
            .key(object_key)
            .send()
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(SdkError::ServiceError(err)) => match err.err() {
                HeadObjectError::NotFound(err) => Err(Error::FileNotFound(err.to_string())),
                err => Err(Error::InternalError(err.to_string())),
            },
            Err(err) => Err(Error::InternalError(err.to_string())),
        }
    }
}
impl MediaStorage for AwsMediaStorage {
    async fn put(
        &self,
        file_path: &path::Path,
        file_name: &str,
        content_type: &str,
        is_private: bool,
    ) -> Result<()> {
        use aws_sdk_s3::primitives::ByteStream;
        let body = ByteStream::from_path(file_path).await.unwrap();

        self.client
            .put_object()
            .body(body)
            .key(file_name)
            .content_type(content_type)
            .acl(if is_private {
                ObjectCannedAcl::Private
            } else {
                ObjectCannedAcl::PublicRead
            })
            .send()
            .await
            .map(|_| ())
            .map_err(|err| Error::UploadFileFailed(err.to_string()))
    }
    async fn get_url(&self, media_id: &str) -> Result<String> {
        self.check_file_exists(media_id).await?;

        let presigned_request = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(media_id)
            .presigned(PresigningConfig::expires_in(self.url_duration).unwrap())
            .await;

        match presigned_request {
            Ok(val) => Ok(val.uri().to_string()),
            Err(err) => Err(Error::InternalError(err.to_string())),
        }
    }

    async fn delete(&self, media_id: &str) -> Result<()> {
        self.check_file_exists(media_id).await?;

        let result = self
            .client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(media_id)
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::InternalError(err.to_string())),
        }
    }
}
