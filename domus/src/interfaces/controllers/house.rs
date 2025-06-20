use actix_web::{post, web, HttpResponse};

use crate::{
    application::{
        commands::{create_house::CreateHouseCommand, update_house::UpdateHouseCommand},
        services::{
            create_house::CreateHouseService, delete_house::DeleteHouseService,
            update_house::UpdateHouseService,
        },
    },
    interfaces::dtos::response::ResponseBody,
};

#[post("/create")]
pub async fn create_house(
    body: web::Json<CreateHouseCommand>,
    create_house_service: web::Data<CreateHouseService>,
) -> HttpResponse {
    let res = match create_house_service.execute(body.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/update")]
pub async fn update_house(
    body: web::Json<UpdateHouseCommand>,
    update_house_service: web::Data<UpdateHouseService>,
) -> HttpResponse {
    let res = match update_house_service.execute(body.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}

#[post("/delete/{house_id}")]
pub async fn delete_house(
    path: web::Path<String>,
    delete_house_service: web::Data<DeleteHouseService>,
) -> HttpResponse {
    let res = match delete_house_service.execute(path.into_inner()).await {
        Ok(data) => ResponseBody::success(data),
        Err(e) => ResponseBody::error(e.to_string()),
    };

    HttpResponse::Ok().json(res)
}
