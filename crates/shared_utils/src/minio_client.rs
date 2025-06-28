use std::time::Duration;

use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, config::Credentials, presigning::PresigningConfig};

pub struct Minio {
    url: Option<String>,
    access_key_id: String,
    secret_access_key: String,
}

impl Minio {
    pub fn new(
        url: Option<String>,
        access_key_id: impl Into<String>,
        secret_access_key: impl Into<String>,
    ) -> Self {
        Self {
            url,
            access_key_id: access_key_id.into(),
            secret_access_key: secret_access_key.into(),
        }
    }

    pub async fn create_client(&self) -> anyhow::Result<MinioClient> {
        let url = self
            .url
            .clone()
            .unwrap_or_else(|| std::env::var("MINIO_URL").unwrap());

        let config = aws_config::defaults(BehaviorVersion::v2025_01_17())
            .region(Region::new("us-east-1"))
            .endpoint_url(&url)
            .credentials_provider(Credentials::new(
                self.access_key_id.clone(),
                self.secret_access_key.clone(),
                None,
                None,
                "static",
            ))
            .load()
            .await;

        let client = Client::new(&config);
        Ok(MinioClient::new(client))
    }
}

pub struct MinioClient {
    client: Client,
}

impl MinioClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    pub async fn get_object_put_url(
        &self,
        bucket: &str,
        key: &str,
        expires_in: Duration,
    ) -> anyhow::Result<String> {
        // 如果 bucket 不存在就创建
        let exists = self
            .client
            .head_bucket()
            .bucket(bucket)
            .send()
            .await
            .is_ok();

        if !exists {
            self.client.create_bucket().bucket(bucket).send().await?;
            tracing::info!("bucket created");
        }

        let req = self
            .client
            .put_object()
            .bucket(bucket)
            .key(key)
            .presigned(PresigningConfig::expires_in(expires_in).unwrap())
            .await?;

        tracing::info!("presigned url: {}", req.uri().to_string());
        Ok(req.uri().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_minio_client() {
        let client = Minio::new(
            Some("http://127.0.0.1:9000".to_string()),
            "minioadmin",
            "minioadmin",
        )
        .create_client()
        .await
        .unwrap();

        let url = client
            .get_object_put_url(
                "domus-houses-images",
                "domus-houses-images/example.txt",
                Duration::from_secs(600),
            )
            .await
            .unwrap();

        println!("url: {}", url);
    }
}
