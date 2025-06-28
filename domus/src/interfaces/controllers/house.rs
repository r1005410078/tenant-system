use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use shared_dto::table_data::TableDataRequest;

use crate::{
    application::{
        commands::{create_house::CreateHouseCommand, update_house::UpdateHouseCommand},
        queries::house::HouseQueryService,
        services::{
            create_house::CreateHouseService, delete_house::DeleteHouseService,
            file_upload_service::FileUploadService, update_house::UpdateHouseService,
        },
    },
    interfaces::dtos::response::ResponseBody,
};

#[post("/create")]
pub async fn create_house(
    body: web::Json<CreateHouseCommand>,
    create_house_service: web::Data<CreateHouseService>,
) -> HttpResponse {
    let res = match create_house_service.execute(body.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/update")]
pub async fn update_house(
    body: web::Json<UpdateHouseCommand>,
    update_house_service: web::Data<UpdateHouseService>,
) -> HttpResponse {
    let res = match update_house_service.execute(body.into_inner()).await {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
struct UploadUrlRequest {
    file_name: String,
}

#[post("/upload-url")]
pub async fn get_upload_url(
    data: web::Json<UploadUrlRequest>,
    file_upload_service: web::Data<FileUploadService>,
) -> HttpResponse {
    let res = match file_upload_service
        .generate_put_url(data.file_name.as_str())
        .await
    {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}
