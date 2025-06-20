mod application;
mod domain;
mod infrastructure;
mod interfaces;
use std::{env, sync::Arc};

use actix_web::{web, App, HttpServer};
use event_bus::AsyncEventBus;

use crate::{
    application::{
        commands::{
            create_community_handler::CreateCommunityCommandHandler,
            create_owner::CreateOwnerCommandHandler,
            delete_community::DeleteCommunityCommandHandler,
            delete_owner::DeleteOwnerCommandHandler,
            update_community_handler::UpdateCommunityCommandHandler,
            update_owner::UpdateOwnerCommandHandler,
        },
        services::{
            create_community::CreateCommunityService, create_owner::CreateOwnerService,
            delete_community::DeleteCommunityService, delete_owner::DeleteOwnerService,
            update_community::UpdateCommunityService, update_owner::UpdateOwnerService,
        },
    },
    infrastructure::{
        community::{
            mysql_community_aggregate::MySqlCommunityAggregateRepository,
            mysql_owner_aggregate::MySqlOwnerAggregateRepository,
        },
        mysql_pool::create_mysql_pool,
    },
    interfaces::controllers::community::{create_community, delete_community, update_community},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::init_tracing();
    dotenvy::dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    let event_bus = Arc::new(AsyncEventBus::new());
    let pool = create_mysql_pool().await;

    // 创建小区仓储
    let community_repo = Arc::new(MySqlCommunityAggregateRepository::new(pool.clone()));

    // 创建小区服务
    let create_community_service = web::Data::new(CreateCommunityService::new(
        CreateCommunityCommandHandler::new(community_repo.clone(), event_bus.clone()),
    ));

    // 更新小区服务
    let update_community_service = web::Data::new(UpdateCommunityService::new(
        UpdateCommunityCommandHandler::new(community_repo.clone(), event_bus.clone()),
    ));

    // 删除小区服务
    let delete_community_service = web::Data::new(DeleteCommunityService::new(
        DeleteCommunityCommandHandler::new(community_repo.clone(), event_bus.clone()),
    ));

    // 创建 拥有者
    let owner_repo = Arc::new(MySqlOwnerAggregateRepository::new(pool.clone()));

    // 创建拥有者服务
    let create_owner_service = web::Data::new(CreateOwnerService::new(
        CreateOwnerCommandHandler::new(owner_repo.clone(), event_bus.clone()),
    ));

    // 更新拥有者服务
    let update_owner_service = web::Data::new(UpdateOwnerService::new(
        UpdateOwnerCommandHandler::new(owner_repo.clone(), event_bus.clone()),
    ));

    // 删除拥有者服务
    let delete_owner_service = web::Data::new(DeleteOwnerService::new(
        DeleteOwnerCommandHandler::new(owner_repo.clone(), event_bus.clone()),
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(create_community_service.clone())
            .app_data(update_community_service.clone())
            .app_data(delete_community_service.clone())
            .app_data(create_owner_service.clone())
            .app_data(update_owner_service.clone())
            .app_data(delete_owner_service.clone())
            .service(
                web::scope("/api/community")
                    .service(create_community)
                    .service(update_community)
                    .service(delete_community),
            )
    })
    .bind(server_url)?
    .run()
    .await
}
