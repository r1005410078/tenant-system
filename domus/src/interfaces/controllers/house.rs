use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use user_system::shared::claims::Claims;

use crate::{
    application::{
        queries::house::{HouseQueryService, HouseRequest},
        services::{
            delete_house::DeleteHouseService,
            file_upload_service::FileUploadService,
            house_operation_log::{HouseOperationLogDto, HouseOperationLogService},
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
    house_operation_log_service: web::Data<HouseOperationLogService>,
    req: HttpRequest,
) -> HttpResponse {
    let extensions = req.extensions();
    let user = extensions.get::<Claims>();

    if user.is_none() {
        return HttpResponse::Forbidden().finish();
    }

    let user = user.unwrap();
    let mut house_command = body.into_inner();
    house_command.update_created_by(user.user_id.clone());

    let ip_address = req
        .connection_info()
        .peer_addr()
        .map(|addr| addr.to_string());

    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let res = match save_house_service.execute(house_command).await {
        Ok(house) => {
            // 记录操作日志
            if let Err(err) = house_operation_log_service
                .save_record(HouseOperationLogDto {
                    operation_type: 0,
                    operation_content: house.clone(),
                    operator_id: user.user_id.clone(),
                    ip_address: ip_address,
                    user_agent: user_agent,
                })
                .await
            {
                tracing::error!("记录操作日志失败:{}", err);
            }

            ResponseBody::success(house)
        }
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

#[post("/list")]
pub async fn list_houses(
    query: web::Json<HouseRequest>,
    house_query_service: web::Data<HouseQueryService>,
) -> HttpResponse {
    let res = match house_query_service.find_all(query.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/group_by_community")]
pub async fn group_by_community(
    house_query_service: web::Data<HouseQueryService>,
    query: web::Json<HouseRequest>,
) -> HttpResponse {
    let res = match house_query_service
        .group_by_community(query.into_inner())
        .await
    {
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
