use actix_web::{get, post, web, HttpResponse};
use shared_dto::table_data::TableDataRequest;

use crate::{
    application::{
        commands::{create_role::CreateRoleCommand, update_role::UpdateRoleCommand},
        queries::role_query_service::RoleQueryService,
        services::{
            create_role::CreateRoleService, delete_role::DeleteRoleService,
            update_role::UpdateRoleService,
        },
    },
    interfaces::dtos::response::ResponseBody,
};

#[post("/create")]
async fn create_role(
    body: web::Json<CreateRoleCommand>,
    service: web::Data<CreateRoleService>,
) -> HttpResponse {
    let res = match service.execute(body.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/delete/{role_id}")]
async fn delete_role(
    path: web::Path<String>,
    delete_role_service: web::Data<DeleteRoleService>,
) -> HttpResponse {
    let res = match delete_role_service.execute(path.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/update")]
async fn update_role(
    body: web::Json<UpdateRoleCommand>,
    service: web::Data<UpdateRoleService>,
) -> HttpResponse {
    let res = match service.execute(body.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[get("/list")]
async fn list_role(
    query: web::Query<TableDataRequest>,
    service: web::Data<RoleQueryService>,
) -> HttpResponse {
    let res = match service.find_all(query.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[get("/detail/{role_id}")]
async fn detail_role(
    path: web::Path<String>,
    service: web::Data<RoleQueryService>,
) -> HttpResponse {
    let res = match service.find(&path.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}
