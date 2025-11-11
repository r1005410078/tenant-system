use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use user_system::shared::claims::Claims;

use crate::{
    application::{
        queries::house::{HouseQueryService, HouseRequest},
        services::{
            delete_house::DeleteHouseService,
            house_operation_log::{HouseOperationLogDto, HouseOperationLogService},
            save_house::SaveHouseService,
        },
    },
    domain::house::value_objects::house::{House, HouseData},
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
    if house_command.house.clone().map(|h| h.id).is_some() {
        house_command.update_created_by(user.user_id.clone());
    }

    let ip_address = req.peer_addr().map(|addr| addr.to_string());
    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // 记录操作日志
    let record_operation_log = async |operation_type, operation_content| {
        if let Err(err) = house_operation_log_service
            .save_record(HouseOperationLogDto {
                operation_type,
                operation_content,
                operator_id: user.user_id.clone(),
                ip_address: ip_address.clone(),
                user_agent: user_agent.clone(),
            })
            .await
        {
            tracing::error!("记录操作日志失败:{}", err);
        }
    };

    // 记录操作日志
    let input_house = house_command.house.clone();
    if input_house.clone().map(|h| h.id).is_some() {
        record_operation_log(2, input_house.clone().unwrap_or_default()).await;
    }

    let res = match save_house_service.execute(house_command).await {
        Ok(house) => {
            if input_house.clone().map(|h| h.id).is_none() {
                record_operation_log(1, house.clone()).await;
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
    house_operation_log_service: web::Data<HouseOperationLogService>,
    req: HttpRequest,
) -> HttpResponse {
    let extensions = req.extensions();
    let user = extensions.get::<Claims>();

    if user.is_none() {
        return HttpResponse::Forbidden().finish();
    }

    let user = user.unwrap();
    let ip_address = req.peer_addr().map(|addr| addr.to_string());
    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let house_id = path.into_inner();
    let res = match delete_house_service.execute(house_id.clone()).await {
        Ok(data) => {
            let mut house = House::default();
            house.id = Some(house_id);

            if let Err(err) = house_operation_log_service
                .save_record(HouseOperationLogDto {
                    operation_type: 3,
                    operation_content: house,
                    operator_id: user.user_id.clone(),
                    ip_address: ip_address.clone(),
                    user_agent: user_agent.clone(),
                })
                .await
            {
                tracing::error!("记录操作日志失败:{}", err);
            }

            ResponseBody::success(data)
        }
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

#[get("/house_operation_log/list/{house_id}")]
pub async fn list_house_operation_log(
    house_id: web::Path<String>,
    house_operation_log_service: web::Data<HouseOperationLogService>,
) -> HttpResponse {
    let res = match house_operation_log_service
        .list(&house_id.into_inner())
        .await
    {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}
