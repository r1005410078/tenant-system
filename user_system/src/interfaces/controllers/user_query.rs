use actix_web::{get, web, HttpResponse};
use shared_dto::table_data::TableDataRequest;

use crate::{
    application::queries::user_query_service::UserQueryService,
    interfaces::dtos::response::ResponseBody,
};

/// 查询用户登陆历史
#[get("/login-history/{user_id}")]
async fn get_login_history(
    path: web::Path<String>,
    table_data_request: web::Query<TableDataRequest>,
    user_query_service: web::Data<UserQueryService>,
) -> HttpResponse {
    let res = match user_query_service
        .get_user_login_history(path.into_inner(), table_data_request.into_inner())
        .await
    {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

// 根据用户id查询用户
#[get("/user_detail/{user_id}")]
async fn get_user(
    path: web::Path<String>,
    user_query_service: web::Data<UserQueryService>,
) -> HttpResponse {
    let res = match user_query_service.find_user(&path.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

// 查询用户列表
#[get("/list")]
async fn get_user_list(
    table_data_request: web::Query<TableDataRequest>,
    user_query_service: web::Data<UserQueryService>,
) -> HttpResponse {
    let res = match user_query_service
        .get_user_list(table_data_request.into_inner())
        .await
    {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}
