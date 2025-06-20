use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::{
    application::{
        commands::{create_owner::CreateOwnerCommand, delete_owner::DeleteOwnerCommand},
        services::create_owner::CreateOwnerService,
    },
    interfaces::dtos::response::ResponseBody,
};

#[post("/create")]
async fn create_owner(
    body: web::Json<CreateOwnerCommand>,
    service: web::Data<CreateOwnerService>,
) -> HttpResponse {
    let res = match service.execute(body.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/update")]
async fn update_owner(
    body: web::Json<crate::application::commands::update_owner::UpdateOwenerCommand>,
    service: web::Data<crate::application::services::update_owner::UpdateOwnerService>,
) -> HttpResponse {
    let res = match service.execute(body.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/delete/{owner_id}")]
async fn delete_owner(
    req: HttpRequest,
    service: web::Data<crate::application::services::delete_owner::DeleteOwnerService>,
) -> HttpResponse {
    let owner_id = req.match_info().get("owner_id").unwrap_or("");
    let command = DeleteOwnerCommand {
        id: owner_id.to_string(),
    };

    let res = match service.execute(command).await {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}
