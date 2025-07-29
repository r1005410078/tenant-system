use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use user_system::shared::claims::Claims;

use crate::{
    application::services::favorite::{
        FavoriteCategories, FavoriteService, UserFavoriteQueryDto, UserFavorites,
    },
    interfaces::dtos::response::ResponseBody,
};

#[post("/favorite_categories/add")]
async fn add_favorite_categories(
    data: web::Json<FavoriteCategories>,
    service: web::Data<FavoriteService>,
    req: HttpRequest,
) -> HttpResponse {
    let extensions = req.extensions();
    let user_id = extensions.get::<Claims>().map(|c| c.user_id.clone());

    if user_id.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let mut data = data.into_inner();
    data.user_id = Some(user_id.unwrap());

    let res = match service.add_favorite_categories(data).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/favorite_categories/delete/{id}")]
async fn delete_favorite_categories(
    id: web::Path<i64>,
    service: web::Data<FavoriteService>,
) -> HttpResponse {
    let res = match service.delete_favorite_categories(id.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[get("/user_favorites/list")]
async fn list_user_favorites(
    service: web::Data<FavoriteService>,
    req: HttpRequest,
) -> HttpResponse {
    let extensions = req.extensions();
    let user_id = extensions.get::<Claims>().map(|c| c.user_id.clone());

    if user_id.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let res = match service.find_user_favorites(user_id.unwrap()).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/user_favorites/add")]
async fn add_user_favorites(
    data: web::Json<UserFavorites>,
    service: web::Data<FavoriteService>,
    req: HttpRequest,
) -> HttpResponse {
    let extensions = req.extensions();
    let user_id = extensions.get::<Claims>().map(|c| c.user_id.clone());

    if user_id.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let mut data = data.into_inner();
    data.user_id = user_id;

    let res = match service.add_user_favorites(data).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/user_favorites/delete/{id}")]
async fn delete_user_favorites(
    id: web::Path<i64>,
    service: web::Data<FavoriteService>,
) -> HttpResponse {
    let res = match service.delete_user_favorites(id.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[get("/user_favorites/list")]
async fn find_user_favorite(
    query: web::Query<UserFavoriteQueryDto>,
    service: web::Data<FavoriteService>,
    req: HttpRequest,
) -> HttpResponse {
    let extensions = req.extensions();
    let user_id = extensions.get::<Claims>().map(|c| c.user_id.clone());

    if user_id.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let mut data = query.into_inner();
    data.user_id = user_id;

    let res = match service.find_user_favorite(data).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}
