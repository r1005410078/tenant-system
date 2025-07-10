use std::{env, sync::Arc};

use actix_web::{web, App, HttpServer};
use event_bus::{AsyncEventBus, EventListener};
use shared_utils::minio_client::Minio;
use tokio::sync::Mutex;
use user_system::shared::{auth_middleware::AuthMiddleware, casbin::init_casbin::init_casbin};

use crate::{
    application::{
        commands::{
            delete_community::DeleteCommunityCommandHandler,
            delete_house::DeleteHouseCommandHandler, delete_owner::DeleteOwnerCommandHandler,
            save_community_handler::SaveCommunityCommandHandler,
            save_house_handler::SaveHouseCommandHandler,
            save_owner_handler::SaveOwnerCommandHandler,
        },
        listeners::{
            community::CommunityEventListener, house::HouseEventListener, owner::OwnerEventListener,
        },
        queries::{
            community::CommunityQueryService, house::HouseQueryService, owner::OwnerQueryService,
        },
        services::{
            delete_community::DeleteCommunityService, delete_house::DeleteHouseService,
            delete_owner::DeleteOwnerService, file_upload_service::FileUploadService,
            save_community::SaveCommunityService, save_house::SaveHouseService,
            save_owner::SaveOwnerService,
        },
    },
    infrastructure::{
        community::mysql_community_aggregate::MySqlCommunityAggregateRepository,
        house::mysql_house_repository_aggregate::MysqlHouseRepositoryAggregate,
        mysql_pool::create_mysql_pool, owner::mysql_owner_aggregate::MySqlOwnerAggregateRepository,
    },
    interfaces::controllers::{
        community::{delete_community, list_community, save_community},
        house::{apply_upload_url, delete_house, get_house_detail, list_houses, save_house},
        owner::{delete_owner, owner_list, save_owner},
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
    let _auth_middleware = Arc::new(AuthMiddleware::new(enforcer.clone()));

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

    // 保存小区服务
    let save_community_command_handler = Arc::new(SaveCommunityCommandHandler::new(
        community_repo.clone(),
        event_bus.clone(),
    ));

    let save_community_service = web::Data::new(SaveCommunityService::new(
        save_community_command_handler.clone(),
    ));

    // 删除小区服务
    let delete_community_service = web::Data::new(DeleteCommunityService::new(
        DeleteCommunityCommandHandler::new(community_repo.clone(), event_bus.clone()),
    ));

    // 创建 业主
    let owner_repo = Arc::new(MySqlOwnerAggregateRepository::new(pool.clone()));

    let save_owner_command_handler = Arc::new(SaveOwnerCommandHandler::new(
        owner_repo.clone(),
        event_bus.clone(),
    ));

    // 保存业主服务
    let save_owner_service =
        web::Data::new(SaveOwnerService::new(save_owner_command_handler.clone()));

    // 删除业主服务
    let delete_owner_service = web::Data::new(DeleteOwnerService::new(
        DeleteOwnerCommandHandler::new(owner_repo.clone(), event_bus.clone()),
    ));

    // 创建房源
    let mysql_house_repository_aggregate =
        Arc::new(MysqlHouseRepositoryAggregate::new(pool.clone()));

    let save_house_command_handler =
        SaveHouseCommandHandler::new(mysql_house_repository_aggregate.clone(), event_bus.clone());

    let save_house_service = web::Data::new(SaveHouseService::new(
        save_house_command_handler,
        save_community_service.clone().into_inner(),
        save_owner_service.clone().into_inner(),
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
            .app_data(save_community_service.clone())
            .app_data(delete_community_service.clone())
            .app_data(save_owner_service.clone())
            .app_data(delete_owner_service.clone())
            .app_data(save_house_service.clone())
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
                            .service(save_community)
                            .service(delete_community),
                    )
                    .service(
                        web::scope("/owner")
                            // .wrap(auth_middleware.clone())
                            .service(save_owner)
                            .service(delete_owner),
                    )
                    .service(
                        web::scope("/house")
                            // .wrap(auth_middleware.clone())
                            .service(save_house)
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
