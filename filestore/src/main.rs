use std::fs::File;
use std::io::Read;

use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::response::PutObjectResponse;
use minio::s3::types::S3Api;

#[tokio::main]
async fn main() {
    let base_url = "http://127.0.0.1:9000".parse::<BaseUrl>().unwrap();
    let static_provider = StaticProvider::new("minioadmin", "minioadmin", None);
    let client = Client::new(base_url, Some(Box::new(static_provider)), None, None).unwrap();

    let exists = client
        .bucket_exists("domus-houses-images")
        .send()
        .await
        .expect("request failed");

    if exists.exists {
        println!("Bucket already exists");

        // 2️⃣ 读取图片文件为字节
        let mut file = File::open("photo.png").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        // 3️⃣ 上传对象
        // let data = bytes::Bytes::from(buffer).into();
        // let resp = client
        //     .put_object("domus-houses-images", "images/photo.png", data)
        //     .send()
        //     .await
        //     .unwrap();

        client
            .delete_object("domus-houses-images", "images/photo.png")
            .send()
            .await
            .unwrap();
    }
}
