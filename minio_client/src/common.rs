use minio::s3::response::BucketExistsResponse;
use minio::s3::types::S3Api;
use minio::s3::Client;

pub async fn create_bucket_if_not_exists(bucket_name: &str, client: &Client) -> anyhow::Result<()> {
    // Check 'bucket_name' bucket exist or not.
    let resp: BucketExistsResponse = client.bucket_exists(bucket_name).send().await?;

    // Make 'bucket_name' bucket if not exist.
    if !resp.exists {
        client.create_bucket(bucket_name).send().await.unwrap();
    };

    Ok(())
}
