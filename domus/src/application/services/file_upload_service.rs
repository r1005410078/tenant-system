use std::sync::Arc;

use shared_utils::minio_client::MinioClient;

pub struct FileUploadService {
    client: Arc<MinioClient>,
}

impl FileUploadService {
    pub fn new(client: Arc<MinioClient>) -> Self {
        Self { client }
    }

    pub async fn generate_put_url(
        &self,
        directory: &str,
        filename: &str,
    ) -> anyhow::Result<String> {
        self.client
            .get_object_put_url(
                "domus-houses-images",
                format!("{}/{}", directory, filename).as_str(),
                std::time::Duration::from_secs(600),
            ) // e.g. 10min
            .await
    }
}
