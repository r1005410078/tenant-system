use actix_web::{get, post, web, HttpResponse};

use crate::{
    application::queries::public_house::{PubclicHouseRequest, PublicHouseQueryService},
    interfaces::dtos::response::ResponseBody,
};

#[post("/list")]
pub async fn list_houses(
    query: web::Json<PubclicHouseRequest>,
    house_query_service: web::Data<PublicHouseQueryService>,
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
    house_query_service: web::Data<PublicHouseQueryService>,
) -> HttpResponse {
    let res = match house_query_service.find_by_id(&path.into_inner()).await {
        Some(data) => ResponseBody::success(data),
        None => ResponseBody::error("not found".to_string()),
    };

    HttpResponse::Ok().json(res)
}
