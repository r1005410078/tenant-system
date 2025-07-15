use actix_web::{get, post, web, HttpRequest, HttpResponse};
use shared_dto::table_data::TableDataRequest;

use crate::{
    application::{
        queries::community::{CommunityQueryService, CommunityRequest},
        services::{
            delete_community::DeleteCommunityService, save_community::SaveCommunityService,
        },
    },
    domain::community::value_objects::commuity::Community,
    interfaces::dtos::response::ResponseBody,
};

#[post("/save")]
async fn save_community(
    body: web::Json<Community>,
    service: web::Data<SaveCommunityService>,
) -> HttpResponse {
    let community = body.into_inner();
    let res = match service.execute(community).await {
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

#[post("/list")]
async fn list_community(
    query: web::Json<CommunityRequest>,
    service: web::Data<CommunityQueryService>,
) -> HttpResponse {
    let res = match service.find_all(query.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}
