use actix_web::{get, post, web, HttpRequest, HttpResponse};
use shared_dto::table_data::TableDataRequest;

use crate::{
    application::{
        commands::delete_owner::DeleteOwnerCommand,
        queries::owner::OwnerQueryService,
        services::{delete_owner::DeleteOwnerService, save_owner::SaveOwnerService},
    },
    domain::owner::value_objects::owner::HouseOwner,
    interfaces::dtos::response::ResponseBody,
};

#[post("/save")]
async fn save_owner(
    body: web::Json<HouseOwner>,
    service: web::Data<SaveOwnerService>,
) -> HttpResponse {
    let res = match service.execute(body.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/delete/{owner_id}")]
async fn delete_owner(req: HttpRequest, service: web::Data<DeleteOwnerService>) -> HttpResponse {
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

#[get("/list")]
async fn owner_list(
    table_data_request: web::Query<TableDataRequest>,
    service: web::Data<OwnerQueryService>,
) -> HttpResponse {
    let res = match service.find_all(table_data_request.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}
