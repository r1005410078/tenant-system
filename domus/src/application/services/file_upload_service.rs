use std::sync::Arc;

use shared_utils::minio_client::MinioClient;

pub struct FileUploadService {
    client: Arc<MinioClient>,
}

impl FileUploadService {
    pub fn new(client: Arc<MinioClient>) -> Self {
        Self { client }
    }

    pub async fn generate_put_url(&self, file_name: &str) -> anyhow::Result<String> {
        println!("file_name: {}", file_name);
        self.client
            .get_object_put_url(
                "domus-houses-images",
                format!("domus-houses-images/{}", file_name).as_str(),
                std::time::Duration::from_secs(600),
            ) // e.g. 10min
            .await
    }
}
