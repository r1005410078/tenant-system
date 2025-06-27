use std::sync::Arc;

use actix_web::web;
use event_bus::{AsyncEventBus, EventListener};
use sea_orm::DbConn;

use crate::{
    application::{
        commands::{
            create_role::CreateRoleCommandHandler,
            init_system::{Cli, InitSystemCommandHandler},
            permission_granted_to_role::PermissionGrantedToRoleCommandHandler,
            register_user::UserRegistrationHandler,
            user_binded_to_roles::UserBindedToRolesHandler,
        },
        listeners::{role::RoleEventListener, user::UserEventListener},
        queries::{role_query_service::RoleQueryService, user_query_service::UserQueryService},
        services::{create_role::CreateRoleService, register_user::RegisterUserService},
    },
    infrastructure::{
        mysql_pool::create_mysql_pool,
        role::{
            mysql_role_query_repository::MysqlRoleQueryRepository,
            role_aggregate_repository::MySqlRoleAggregateRepository,
        },
        user::{
            mysq_user_query_repository::MysqlUserQueryRepository,
            user_aggregate_repository::MySqlUserAggregateRepository,
        },
    },
};

pub async fn execute(admin_name: String, admin_password: String) -> std::io::Result<()> {
    let pool = create_mysql_pool().await;
    let event_bus = Arc::new(AsyncEventBus::new(Some(pool.clone())));

    // 创建服务
    let ServiceProvider {
        register_user_services,
        create_role_services,
    } = ServiceProvider::config(pool.clone(), event_bus.clone());

    // 注册事件
    register_event_handlers(pool.clone(), event_bus.clone());

    // 执行初始化逻辑
    tracing::info!("开始初始化系统...");
    // 假设有 InitSystemHandler
    let handler =
        InitSystemCommandHandler::new(pool.clone(), register_user_services, create_role_services);

    handler.handle(admin_name, admin_password).await.unwrap();
    tracing::info!("初始化完成！");

    Ok(())
}

struct ServiceProvider {
    pub register_user_services: RegisterUserService,
    pub create_role_services: CreateRoleService,
}

impl ServiceProvider {
    pub fn config(pool: Arc<DbConn>, event_bus: Arc<AsyncEventBus>) -> Self {
        // 创建用户仓储
        let user_repo = Arc::new(MySqlUserAggregateRepository::new(pool.clone()));

        // 用户绑定角色
        let user_binded_to_roles_command_handler = Arc::new(UserBindedToRolesHandler::new(
            user_repo.clone(),
            event_bus.clone(),
        ));

        // 注册用户
        let register_user_services = RegisterUserService::new(
            UserRegistrationHandler::new(event_bus.clone(), user_repo.clone()),
            user_binded_to_roles_command_handler.clone(),
        );

        // 角色仓储层
        let role_repo = Arc::new(MySqlRoleAggregateRepository::new(pool.clone()));
        let permission_granted_to_role_command_handler = Arc::new(
            PermissionGrantedToRoleCommandHandler::new(role_repo.clone(), event_bus.clone()),
        );

        // 创建角色
        let create_role_services = CreateRoleService::new(
            CreateRoleCommandHandler::new(role_repo.clone(), event_bus.clone()),
            permission_granted_to_role_command_handler.clone(),
        );

        Self {
            register_user_services,
            create_role_services,
        }
    }
}

fn register_event_handlers(pool: Arc<DbConn>, event_bus: Arc<AsyncEventBus>) {
    // 用户读模型仓储
    let user_read_repo = Arc::new(MysqlUserQueryRepository::new(pool.clone()));
    // 用户读模型服务
    let user_query_service = web::Data::new(UserQueryService::new(user_read_repo));

    // 注册用户绑定角色事件
    Arc::new(UserEventListener::new(
        user_query_service.clone().into_inner(),
    ))
    .subscribe(event_bus.clone());

    // 角色仓储
    let mysql_role_query_repository = Arc::new(MysqlRoleQueryRepository::new(pool.clone()));
    // 角色服务
    let role_query_service =
        web::Data::new(RoleQueryService::new(mysql_role_query_repository.clone()));

    // 注册角色事件
    Arc::new(RoleEventListener::new(
        role_query_service.clone().into_inner(),
    ))
    .subscribe(event_bus.clone());
}
