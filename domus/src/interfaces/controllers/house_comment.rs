use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use user_system::shared::claims::Claims;

use crate::{
    application::services::house_comment::HouseCommentService,
    interfaces::dtos::response::ResponseBody,
};

#[derive(Deserialize, Serialize, Clone)]
struct AddCommentRequest {
    house_id: String,
    comment: String,
}

#[post("/add_comment")]
pub async fn add_comment(
    req: HttpRequest,
    body: web::Json<AddCommentRequest>,
    house_comment_service: web::Data<HouseCommentService>,
) -> HttpResponse {
    let extensions = req.extensions();
    let admin = extensions
        .get::<Claims>()
        .map(|c| c.username.clone())
        .unwrap_or("test".to_string());

    let res = match house_comment_service
        .add_comment(&admin, &body.house_id, &body.comment)
        .await
    {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct UpdateCommentRequest {
    comment_id: String,
    comment: String,
}

#[post("/update_comment")]
pub async fn update_comment(
    req: HttpRequest,
    body: web::Json<UpdateCommentRequest>,
    house_comment_service: web::Data<HouseCommentService>,
) -> HttpResponse {
    let extensions = req.extensions();
    let admin = extensions
        .get::<Claims>()
        .map(|c| c.username.clone())
        .unwrap_or("test".to_string());

    let res = match house_comment_service
        .update_comment(&admin, &body.comment_id, &body.comment)
        .await
    {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/delete_comment/{comment_id}")]
pub async fn delete_comment(
    req: HttpRequest,
    comment_id: web::Path<String>,
    house_comment_service: web::Data<HouseCommentService>,
) -> HttpResponse {
    let extensions = req.extensions();
    let admin = extensions
        .get::<Claims>()
        .map(|c| c.username.clone())
        .unwrap_or("test".to_string());

    let comment_id = comment_id.into_inner();
    let res = match house_comment_service
        .delete_comment(&admin, &comment_id)
        .await
    {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[get("/get_comments/{house_id}")]
pub async fn get_comments(
    house_id: web::Path<String>,
    house_comment_service: web::Data<HouseCommentService>,
) -> HttpResponse {
    let house_id = house_id.into_inner();
    let res = match house_comment_service.get_comments(&house_id).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}
