use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use shared_dto::table_data::TableDataRequest;

use crate::{
    application::{
        queries::house::HouseQueryService,
        services::{
            delete_house::DeleteHouseService, file_upload_service::FileUploadService,
            save_house::SaveHouseService,
        },
    },
    domain::house::value_objects::house::HouseData,
    interfaces::dtos::response::ResponseBody,
};

#[post("/save")]
pub async fn save_house(
    body: web::Json<HouseData>,
    save_house_service: web::Data<SaveHouseService>,
) -> HttpResponse {
    let res = match save_house_service.execute(body.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/delete/{house_id}")]
pub async fn delete_house(
    path: web::Path<String>,
    delete_house_service: web::Data<DeleteHouseService>,
) -> HttpResponse {
    let res = match delete_house_service.execute(path.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[get("/list")]
pub async fn list_houses(
    query: web::Query<TableDataRequest>,
    house_query_service: web::Data<HouseQueryService>,
) -> HttpResponse {
    let res = match house_query_service.find_all(query.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[get("/detail/{house_id}")]
pub async fn get_house_detail(
    path: web::Path<String>,
    house_query_service: web::Data<HouseQueryService>,
) -> HttpResponse {
    let res = match house_query_service.find_by_id(&path.into_inner()).await {
        Some(data) => ResponseBody::success(data),
        None => ResponseBody::error("not found".to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct UploadUrlRequest {
    directory: String,
    filename: String,
}

#[post("/apply_upload_url")]
pub async fn apply_upload_url(
    data: web::Json<UploadUrlRequest>,
    file_upload_service: web::Data<FileUploadService>,
) -> HttpResponse {
    let res = match file_upload_service
        .generate_put_url(data.directory.as_str(), data.filename.as_str())
        .await
    {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}
