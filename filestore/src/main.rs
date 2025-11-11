use crate::{
    api::upload::{get_house_media_resource_path, upload_house_media},
    service::upload_house_images::UploadHouseMediaResourceService,
};
use actix_multipart::form::MultipartFormConfig;
use actix_web::{App, HttpServer, web};
use minio::s3::{Client, creds::StaticProvider, http::BaseUrl};
use std::env;
// use user_system::shared::{auth_middleware::AuthMiddleware, casbin::init_casbin::init_casbin};

pub mod api;
pub mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::init_tracing();
    dotenvy::dotenv().ok();

    let host = env::var("HOST").unwrap_or("0.0.0.0".into());
    let port = env::var("FILE_STORE_PORT").unwrap_or("9003".into());
    let server_url = format!("{host}:{port}");

    let minio_url = env::var("MINIO_URL").unwrap_or("http://127.0.0.1:9000".into());
    println!("minio_url: {}", minio_url);
    let minio_url = minio_url.parse::<BaseUrl>().unwrap();
    let static_provider = StaticProvider::new("minioadmin", "minioadmin", None);
    let client = Client::new(minio_url, Some(Box::new(static_provider)), None, None).unwrap();

    let upload_house_media_resource_service =
        web::Data::new(UploadHouseMediaResourceService::new(client));

    // let enforcer = Arc::new(Mutex::new(init_casbin().await));
    // let auth_middleware = Arc::new(AuthMiddleware::new(enforcer.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(MultipartFormConfig::default().total_limit(100 * 1024 * 1024))
            .app_data(upload_house_media_resource_service.clone())
            .service(
                web::scope("/api/filestore")
                    // .wrap(auth_middleware.clone())
                    .service(upload_house_media)
                    .service(get_house_media_resource_path),
            )
    })
    .workers(2)
    .bind(server_url)?
    .run()
    .await
}
