use std::{env, sync::Arc};

use actix_web::{web, App, HttpServer};
use event_bus::{AsyncEventBus, EventListener};
use shared_utils::minio_client::Minio;
use tokio::sync::Mutex;
use user_system::shared::{auth_middleware::AuthMiddleware, casbin::init_casbin::init_casbin};

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
        listeners::{
            community::CommunityEventListener, house::HouseEventListener, owner::OwnerEventListener,
        },
        queries::{
            community::CommunityQueryService, house::HouseQueryService, owner::OwnerQueryService,
        },
        services::{
            create_community::CreateCommunityService, create_house::CreateHouseService,
            create_owner::CreateOwnerService, delete_community::DeleteCommunityService,
            delete_house::DeleteHouseService, delete_owner::DeleteOwnerService,
            file_upload_service::FileUploadService, save_community::SaveCommunityService,
            save_owner::SaveOwnerService, update_community::UpdateCommunityService,
            update_house::UpdateHouseService, update_owner::UpdateOwnerService,
        },
    },
    infrastructure::{
        community::mysql_community_aggregate::MySqlCommunityAggregateRepository,
        house::mysql_house_repository_aggregate::MysqlHouseRepositoryAggregate,
        mysql_pool::create_mysql_pool, owner::mysql_owner_aggregate::MySqlOwnerAggregateRepository,
    },
    interfaces::controllers::{
        community::{create_community, delete_community, list_community, update_community},
        house::{
            apply_upload_url, create_house, delete_house, get_house_detail, list_houses,
            update_house,
        },
        owner::{create_owner, delete_owner, owner_list, update_owner},
    },
};

pub async fn execute() -> std::io::Result<()> {
    log::init_tracing();
    dotenvy::dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("DOMUS_PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    let pool = create_mysql_pool().await;
    let event_bus = Arc::new(AsyncEventBus::new(Some(pool.clone())));
    let enforcer = Arc::new(Mutex::new(init_casbin().await));
    let auth_middleware = Arc::new(AuthMiddleware::new(enforcer.clone()));

    // minio_client
    let minio_client = Arc::new(
        Minio::new(
            Some("http://127.0.0.1:9000".to_string()),
            "minioadmin",
            "minioadmin",
        )
        .create_client()
        .await
        .unwrap(),
    );

    // 上传文件服务
    let file_upload_service = web::Data::new(FileUploadService::new(minio_client.clone()));

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
    let community_query_service = web::Data::new(CommunityQueryService::new(pool.clone()));
    // 小区事件
    Arc::new(CommunityEventListener::new(
        community_query_service.clone().into_inner(),
    ))
    .subscribe(event_bus.clone());

    // 业主仓储
    let owner_query_service = web::Data::new(OwnerQueryService::new(pool.clone()));
    // 业主事件
    Arc::new(OwnerEventListener::new(
        owner_query_service.clone().into_inner(),
    ))
    .subscribe(event_bus.clone());

    // 房源仓储
    let house_query_service = web::Data::new(HouseQueryService::new(pool.clone()));
    // 房源事件
    Arc::new(HouseEventListener::new(
        house_query_service.clone().into_inner(),
    ))
    .subscribe(event_bus.clone());

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
            .app_data(community_query_service.clone())
            .app_data(owner_query_service.clone())
            .app_data(house_query_service.clone())
            .app_data(file_upload_service.clone())
            .service(
                web::scope("/api/domus/management")
                    .service(
                        web::scope("/community")
                            //.wrap(auth_middleware.clone())
                            .service(create_community)
                            .service(update_community)
                            .service(delete_community),
                    )
                    .service(
                        web::scope("/owner")
                            // .wrap(auth_middleware.clone())
                            .service(create_owner)
                            .service(update_owner)
                            .service(delete_owner),
                    )
                    .service(
                        web::scope("/house")
                            // .wrap(auth_middleware.clone())
                            .service(create_house)
                            .service(update_house)
                            .service(delete_house)
                            .service(apply_upload_url),
                    ),
            )
            .service(
                web::scope("/api/domus/query")
                    // .wrap(auth_middleware.clone())
                    .service(web::scope("/owner").service(owner_list))
                    .service(web::scope("/community").service(list_community))
                    .service(
                        web::scope("/house")
                            .service(list_houses)
                            .service(get_house_detail),
                    ),
            )
    })
    .bind(server_url)?
    .run()
    .await
}
