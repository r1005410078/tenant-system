use std::time::Duration;

use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{
    Client,
    config::{
        Credentials,
        endpoint::{Endpoint, EndpointFuture, Params, ResolveEndpoint},
    },
    presigning::PresigningConfig,
    primitives::ByteStream,
};

pub struct Minio {
    url: Option<String>,
    access_key_id: String,
    secret_access_key: String,
}

#[derive(Debug)]
struct MinioResolver {
    url: String,
}

impl ResolveEndpoint for MinioResolver {
    fn resolve_endpoint(&self, _params: &Params) -> EndpointFuture<'_> {
        println!("url: {}", self.url);
        // 构建 Endpoint
        EndpointFuture::ready(Ok(Endpoint::builder().url(self.url.clone()).build()))
    }
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
            .unwrap_or_else(|| std::env::var("MINIO_URL").unwrap_or("192.168.1.10:9000".into()));

        // let config = aws_config::defaults(BehaviorVersion::v2025_08_07())
        //     .region(Region::new("us-east-1"))
        //     .endpoint_url(&url)
        //     .credentials_provider(Credentials::new(
        //         self.access_key_id.clone(),
        //         self.secret_access_key.clone(),
        //         None,
        //         None,
        //         "static",
        //     ))
        //     .load()
        //     .await;

        let creds = Credentials::new(
            self.access_key_id.clone(),
            self.secret_access_key.clone(),
            None,
            None,
            "static",
        );

        // 构建 endpoint
        let resolver = MinioResolver { url };
        let config = aws_sdk_s3::Config::builder()
            .region(Region::new("us-east-1"))
            .credentials_provider(creds)
            .force_path_style(false)
            .endpoint_resolver(resolver)
            .behavior_version(BehaviorVersion::v2025_08_07())
            .build();

        let client = aws_sdk_s3::Client::from_conf(config);

        // 假设对象内容是 &str
        let content = "hello world";

        // 转成 ByteStream
        let body = ByteStream::from(content.as_bytes().to_vec());
        client
            .put_object()
            .bucket("domus-houses-images")
            .key("example.txt") // 对象名
            .body(body)
            .send()
            .await?;

        let bucket_name = "domus-houses-images";
        // 检查桶是否存在
        match client.head_bucket().bucket(bucket_name).send().await {
            Ok(_) => true,
            Err(e) => {
                println!("Bucket {} not found: {:?}", bucket_name, e);
                false
            }
        };

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
        // let exists = self.client.head_bucket().bucket(bucket).send().await;
        match self.client.head_bucket().bucket(bucket).send().await {
            Ok(_) => println!("Bucket {} exists", bucket),
            Err(_) => {
                self.client.create_bucket().bucket(bucket).send().await?;
                tracing::info!("bucket created");
            }
        }

        let req = self
            .client
            .put_object()
            .bucket(bucket)
            .key(key)
            .presigned(PresigningConfig::expires_in(expires_in).unwrap())
            .await?;

        Ok(req.uri().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_minio_client() {
        let client = Minio::new(
            Some("http://192.168.2.100:9000".to_string()),
            "meida",
            "rts2778205",
        )
        .create_client()
        .await
        .unwrap();

        let url = client
            .get_object_put_url("meida", "meida/example.txt", Duration::from_secs(600))
            .await
            .unwrap();

        println!("url: {}", url);
    }
}
