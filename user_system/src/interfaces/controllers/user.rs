use actix_web::{post, web, HttpResponse};

use crate::{
    application::{
        commands::{
            login::LoginCommand, register_user::RegisterUserCommand, update_user::UpdateUserCommand,
        },
        services::{
            delete_user::DeleteUserService, login::LoginService,
            register_user::RegisterUserService, update_user::UpdateUserService,
        },
    },
    interfaces::dtos::response::ResponseBody,
};

// 用户注册
#[post("/register")]
async fn register(
    user: web::Json<RegisterUserCommand>,
    register_service: web::Data<RegisterUserService>,
) -> HttpResponse {
    let res = match register_service.execute(user.into_inner()).await {
        Ok(data) => ResponseBody::success(data.id),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

// 删除用户
#[post("/delete/{user_id}")]
async fn delete_user(
    path: web::Path<String>,
    delete_user_service: web::Data<DeleteUserService>,
) -> HttpResponse {
    let res = match delete_user_service.execute(path.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

// 更新用户
#[post("/update")]
async fn update_user(
    user: web::Json<UpdateUserCommand>,
    update_user_service: web::Data<UpdateUserService>,
) -> HttpResponse {
    let res = match update_user_service.execute(user.into_inner()).await {
        Ok(data) => ResponseBody::success(data.id),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

// 登陆用户
#[post("/login")]
async fn login(
    user: web::Json<LoginCommand>,
    login_service: web::Data<LoginService>,
) -> HttpResponse {
    let res = match login_service.execute(user.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(err) => ResponseBody::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}
