use actix_web::{post, web, HttpResponse};

use crate::{
    application::{
        commands::{create_role::CreateRoleCommand, update_role::UpdateRoleCommand},
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
