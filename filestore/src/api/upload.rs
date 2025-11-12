use actix_web::{HttpResponse, Responder, get, post, web};

use crate::{
    api::dtos::response::ResponseBody,
    service::upload_house_images::UploadHouseMediaResourceService,
};
use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};
use serde::Deserialize;
use std::{env, io::Read, thread};

#[derive(Debug, Deserialize)]
struct Metadata {
    name: String,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
    json: MpJson<Metadata>,
}

#[post("/upload_house_media")]
pub async fn upload_house_media(
    service: web::Data<UploadHouseMediaResourceService>,
    MultipartForm(mut form): MultipartForm<UploadForm>,
) -> impl Responder {
    let mut contents = vec![];
    form.file.file.read_to_end(&mut contents).unwrap();

    let service = service.into_inner().clone();

    let filename = form.json.name.clone();

    let handle = thread::spawn(move || {
        // 创建一个单线程或多线程的 Tokio runtime
        let rt = tokio::runtime::Runtime::new().unwrap();

        // 在 runtime 上 block_on 运行 async 任务
        rt.block_on(async {
            if let Err(err) = service.upload(contents, filename).await {
                println!("上传失败 {}", err);
            }
        })
    });

    // 阻塞等待线程完成
    handle.join().unwrap();

    HttpResponse::Ok().json(ResponseBody::success("".to_string()))
}

// 获取资源路径 bucket
#[get("/get_house_media_resource_path")]
pub async fn get_house_media_resource_path() -> impl Responder {
    let minio_url = env::var("MINIO_URL").unwrap_or("http://127.0.0.1:9000".into());
    HttpResponse::Ok().json(ResponseBody::success(format!(
        "{}/domus-houses-images/",
        minio_url
    )))
}
