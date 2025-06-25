use std::path::Path;
use std::time::Duration;

use http::Method;
use minio::s3::builders::ObjectContent;
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::response::{
    BucketExistsResponse, CreateBucketResponse, GetObjectPromptResponse, GetObjectResponse,
    GetPresignedObjectUrlResponse, PutObjectResponse,
};
use minio::s3::segmented_bytes::SegmentedBytes;
use minio::s3::types::S3Api;

use crate::common::create_bucket_if_not_exists;

mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let base_url: BaseUrl = "http://localhost:9000".parse().unwrap();
    let provider = StaticProvider::new("minioadmin", "minioadmin", None);
    let client = Client::new(base_url, Some(Box::new(provider)), None, None)?;

    let bucket_name: &str = "file-upload-rust-bucket";
    create_bucket_if_not_exists(bucket_name, &client).await?;

    let filename: &Path = Path::new("./examples/cat.png");

    // Name of the object that will be stored in the bucket
    let object_name: &str = "cat.png";

    if filename.exists() {
        println!("File '{}' exists.", &filename.to_str().unwrap());
    } else {
        println!("File '{}' does not exist.", &filename.to_str().unwrap());
        return Ok(());
    }

    let content = ObjectContent::from(filename);
    client
        .put_object_content(bucket_name, object_name, content)
        .send()
        .await?;

    println!(
        "file '{}' is successfully uploaded as object '{object_name}' to bucket '{bucket_name}'.",
        filename.display()
    );

    println!("Presigned URL: http://localhost:9000/file-upload-rust-bucket/cat.png");

    Ok(())
}
