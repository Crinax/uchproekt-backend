use std::fmt::Debug;

use s3::{
    creds::{error::CredentialsError, Credentials},
    error::S3Error,
    Bucket, Region,
};

pub trait S3Credentials {
    fn get_s3_region(&self) -> String;
    fn get_s3_endpoint(&self) -> String;
    fn get_s3_user(&self) -> &str;
    fn get_s3_password(&self) -> &str;
    fn get_s3_bucket(&self) -> &str;
}

pub enum S3AdapterError<CErr, BErr> {
    CredentialsError(CErr),
    BucketError(BErr),
}

pub struct S3Adapter {
    bucket: Bucket,
}

impl S3Adapter {
    pub fn new<T: S3Credentials>(
        config: T,
    ) -> Result<Self, S3AdapterError<CredentialsError, S3Error>> {
        let region = Region::Custom {
            region: config.get_s3_region(),
            endpoint: config.get_s3_endpoint(),
        };

        let credentials = Credentials::new(
            Some(config.get_s3_user()),
            Some(config.get_s3_password()),
            None,
            None,
            None,
        )
        .map_err(|err| {
            log::error!("Credentials error: {:?}", err);
            S3AdapterError::CredentialsError(err)
        })?;

        Ok(Self {
            bucket: Bucket::new(config.get_s3_bucket(), region, credentials).map_err(|err| {
                log::error!("Bucket error: {:?}", err);
                S3AdapterError::BucketError(err)
            })?,
        })
    }

    pub async fn get_object(&self, key: &str) -> _ {
        self.bucket.get_object(key).await
    }
}
