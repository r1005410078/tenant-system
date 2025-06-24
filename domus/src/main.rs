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
            create_house_handler::CreateHouseCommandHandler,
            create_owner::CreateOwnerCommandHandler,
            delete_community::DeleteCommunityCommandHandler,
            delete_house::DeleteHouseCommandHandler, delete_owner::DeleteOwnerCommandHandler,
            update_community_handler::UpdateCommunityCommandHandler,
            update_house_handler::UpdateHouseCommandHandler,
            update_owner::UpdateOwnerCommandHandler,
        },
        services::{
            create_community::CreateCommunityService, create_house::CreateHouseService,
            create_owner::CreateOwnerService, delete_community::DeleteCommunityService,
            delete_house::DeleteHouseService, delete_owner::DeleteOwnerService,
            save_community::SaveCommunityService, save_owner::SaveOwnerService,
            update_community::UpdateCommunityService, update_house::UpdateHouseService,
            update_owner::UpdateOwnerService,
        },
    },
    infrastructure::{
        community::mysql_community_aggregate::MySqlCommunityAggregateRepository,
        house::mysql_house_repository_aggregate::{self, MysqlHouseRepositoryAggregate},
        mysql_pool::create_mysql_pool,
        owner::mysql_owner_aggregate::MySqlOwnerAggregateRepository,
    },
    interfaces::controllers::{
        community::{create_community, delete_community, update_community},
        house::{create_house, delete_house, update_house},
        owner::{create_owner, delete_owner, update_owner},
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::init_tracing();
    dotenvy::dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    let pool = create_mysql_pool().await;
    let event_bus = Arc::new(AsyncEventBus::new(Some(pool.clone())));

    // 创建小区仓储
    let community_repo = Arc::new(MySqlCommunityAggregateRepository::new(pool.clone()));

    // 创建小区服务
    let create_community_command_handler = Arc::new(CreateCommunityCommandHandler::new(
        community_repo.clone(),
        event_bus.clone(),
    ));

    let create_community_service = web::Data::new(CreateCommunityService::new(
        create_community_command_handler.clone(),
    ));

    // 更新小区服务
    let update_community_command_handler = Arc::new(UpdateCommunityCommandHandler::new(
        community_repo.clone(),
        event_bus.clone(),
    ));

    let update_community_service = web::Data::new(UpdateCommunityService::new(
        update_community_command_handler.clone(),
    ));

    // 删除小区服务
    let delete_community_service = web::Data::new(DeleteCommunityService::new(
        DeleteCommunityCommandHandler::new(community_repo.clone(), event_bus.clone()),
    ));

    // 创建 业主
    let owner_repo = Arc::new(MySqlOwnerAggregateRepository::new(pool.clone()));

    let create_owner_command_handler = Arc::new(CreateOwnerCommandHandler::new(
        owner_repo.clone(),
        event_bus.clone(),
    ));

    // 创建业主服务
    let create_owner_service = web::Data::new(CreateOwnerService::new(
        create_owner_command_handler.clone(),
    ));

    let update_owner_command_handler = Arc::new(UpdateOwnerCommandHandler::new(
        owner_repo.clone(),
        event_bus.clone(),
    ));

    // 更新业主服务
    let update_owner_service = web::Data::new(UpdateOwnerService::new(
        update_owner_command_handler.clone(),
    ));

    // 删除业主服务
    let delete_owner_service = web::Data::new(DeleteOwnerService::new(
        DeleteOwnerCommandHandler::new(owner_repo.clone(), event_bus.clone()),
    ));

    let save_community_service = Arc::new(SaveCommunityService::new(
        create_community_command_handler.clone(),
        update_community_command_handler.clone(),
    ));

    let save_owner_service = Arc::new(SaveOwnerService::new(
        create_owner_command_handler.clone(),
        update_owner_command_handler.clone(),
    ));

    // 创建房源
    let mysql_house_repository_aggregate =
        Arc::new(MysqlHouseRepositoryAggregate::new(pool.clone()));

    let create_house_service = web::Data::new(CreateHouseService::new(
        CreateHouseCommandHandler::new(mysql_house_repository_aggregate.clone(), event_bus.clone()),
        save_community_service.clone(),
        save_owner_service.clone(),
    ));

    let update_house_service = web::Data::new(UpdateHouseService::new(
        UpdateHouseCommandHandler::new(mysql_house_repository_aggregate.clone(), event_bus.clone()),
        save_community_service.clone(),
        save_owner_service.clone(),
    ));

    // 删除房源
    let delete_house_service = web::Data::new(DeleteHouseService::new(
        DeleteHouseCommandHandler::new(mysql_house_repository_aggregate.clone(), event_bus.clone()),
    ));

    // 小区仓储
    // let community_query_repo = Arc::new(MySqlCommunityAggregateRepository::new(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(create_community_service.clone())
            .app_data(update_community_service.clone())
            .app_data(delete_community_service.clone())
            .app_data(create_owner_service.clone())
            .app_data(update_owner_service.clone())
            .app_data(delete_owner_service.clone())
            .app_data(create_house_service.clone())
            .app_data(update_house_service.clone())
            .app_data(delete_house_service.clone())
            .service(
                web::scope("/api/community")
                    .service(create_community)
                    .service(update_community)
                    .service(delete_community),
            )
            .service(
                web::scope("/api/owner")
                    .service(create_owner)
                    .service(update_owner)
                    .service(delete_owner),
            )
            .service(
                web::scope("/api/house")
                    .service(create_house)
                    .service(update_house)
                    .service(delete_house),
            )
    })
    .bind(server_url)?
    .run()
    .await
}
