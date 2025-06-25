use actix_web::{get, post, web, HttpRequest, HttpResponse};
use shared_dto::table_data::TableDataRequest;

use crate::{
    application::{
        commands::{
            create_community::CreateCommunityCommand, update_community::UpdateCommunityCommand,
        },
        queries::community::CommunityQueryService,
        services::{
            create_community::CreateCommunityService, delete_community::DeleteCommunityService,
            update_community::UpdateCommunityService,
        },
    },
    interfaces::dtos::response::ResponseBody,
};

#[post("/create")]
async fn create_community(
    body: web::Json<CreateCommunityCommand>,
    service: web::Data<CreateCommunityService>,
) -> HttpResponse {
    let command = body.into_inner();
    let res = match service.execute(command).await {
        Ok(community) => ResponseBody::success(community),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/update")]
async fn update_community(
    body: web::Json<UpdateCommunityCommand>,
    service: web::Data<UpdateCommunityService>,
) -> HttpResponse {
    let command = body.into_inner();
    let res = match service.execute(command).await {
        Ok(community) => ResponseBody::success(community),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/delete/{community_id}")]
async fn delete_community(
    req: HttpRequest,
    service: web::Data<DeleteCommunityService>,
) -> HttpResponse {
    let community_id = req.match_info().get("community_id").unwrap_or("");
    let res = match service.execute(community_id.to_string()).await {
        Ok(()) => ResponseBody::success(()),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[get("/list")]
async fn list_community(
    query: web::Query<TableDataRequest>,
    service: web::Data<CommunityQueryService>,
) -> HttpResponse {
    let res = match service.find_all(query.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}
