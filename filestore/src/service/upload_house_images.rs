use minio::s3::{client::Client, types::S3Api};

static MEIDA_HOUSE_IMAGES_BUCKET: &str = "domus-houses-images";

// 上传美大房源多媒体资源
pub struct UploadHouseMediaResourceService {
    client: Client,
}

impl UploadHouseMediaResourceService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn upload(&self, buffer: Vec<u8>, filename: String) -> anyhow::Result<()> {
        let exists = self
            .client
            .bucket_exists(MEIDA_HOUSE_IMAGES_BUCKET)
            .send()
            .await?;

        if !exists.exists {
            self.client
                .create_bucket(MEIDA_HOUSE_IMAGES_BUCKET)
                .send()
                .await?;
        }

        let data = bytes::Bytes::from(buffer).into();
        self.client
            .put_object(MEIDA_HOUSE_IMAGES_BUCKET, filename, data)
            .send()
            .await?;

        Ok(())
    }
}
