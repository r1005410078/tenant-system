mod application;
mod domain;
mod infrastructure;
mod interfaces;

use std::{env, sync::Arc};

use crate::{
    application::{
        commands::{
            create_role::CreateRoleCommandHandler, delete_role::DeleteRoleCommandHandler,
            delete_user::DeleteUserCommandHandler, login::LoginCommandHandler,
            permission_granted_to_role::PermissionGrantedToRoleCommandHandler,
            register_user::UserRegistrationHandler, update_role::UpdateRoleCommandHandler,
            update_user::UpdateUserCommandHandler, user_binded_to_roles::UserBindedToRolesHandler,
        },
        listeners::{login::LoginEventListener, user::UserEventListener},
        queries::user_query_service::UserQueryService,
        services::{
            create_role::CreateRoleService, delete_role::DeleteRoleService,
            delete_user::DeleteUserService, login::LoginService,
            register_user::RegisterUserService, update_role::UpdateRoleService,
            update_user::UpdateUserService,
        },
    },
    infrastructure::{
        casbin::init_casbin::init_casbin,
        mysql_pool::create_mysql_pool,
        role::role_aggregate_repository::MySqlRoleAggregateRepository,
        user::{
            mysq_user_query_repository::MysqlUserQueryRepository,
            user_aggregate_repository::MySqlUserAggregateRepository,
        },
    },
    interfaces::controllers::{
        role::{create_role, delete_role, update_role},
        user::{delete_user, login, register, update_user},
        user_query::get_login_history,
    },
};
use actix_web::{web, App, HttpServer};
use event_bus::{AsyncEventBus, EventListener};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::init_tracing();
    dotenvy::dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    let event_bus = Arc::new(AsyncEventBus::new());
    let pool = create_mysql_pool().await;
    let enforcer = Arc::new(init_casbin(pool.clone()).await);

    // 创建用户仓储
    let user_repo = Arc::new(MySqlUserAggregateRepository::new(pool.clone()));

    // 用户绑定角色
    let user_binded_to_roles_command_handler = Arc::new(UserBindedToRolesHandler::new(
        user_repo.clone(),
        event_bus.clone(),
    ));

    // 注册用户
    let register_user_services = web::Data::new(RegisterUserService::new(
        UserRegistrationHandler::new(event_bus.clone(), user_repo.clone()),
        user_binded_to_roles_command_handler.clone(),
    ));

    // 更新用户
    let update_user_services = web::Data::new(UpdateUserService::new(
        UpdateUserCommandHandler::new(user_repo.clone(), event_bus.clone()),
        user_binded_to_roles_command_handler.clone(),
    ));

    // 删除用户
    let delete_user_services = web::Data::new(DeleteUserService::new(
        DeleteUserCommandHandler::new(user_repo.clone(), event_bus.clone()),
    ));

    // 登录用户
    let login_services = web::Data::new(LoginService::new(LoginCommandHandler::new(
        user_repo.clone(),
        event_bus.clone(),
    )));

    // 角色仓储层
    let role_repo = Arc::new(MySqlRoleAggregateRepository::new(pool.clone()));

    // 创建角色
    let create_role_services = web::Data::new(CreateRoleService::new(
        CreateRoleCommandHandler::new(role_repo.clone(), event_bus.clone()),
    ));

    // 删除角色
    let delete_role_services = web::Data::new(DeleteRoleService::new(
        DeleteRoleCommandHandler::new(role_repo.clone(), event_bus.clone()),
    ));

    // 更新角色
    let update_role_services = web::Data::new(UpdateRoleService::new(
        UpdateRoleCommandHandler::new(role_repo.clone(), event_bus.clone()),
        PermissionGrantedToRoleCommandHandler::new(role_repo.clone(), event_bus.clone()),
    ));

    // 用户读模型仓储
    let user_read_repo = Arc::new(MysqlUserQueryRepository::new(pool.clone()));
    // 用户读模型服务
    let user_query_service = web::Data::new(UserQueryService::new(user_read_repo));

    // 注册登陆事件
    Arc::new(LoginEventListener::new(
        user_query_service.clone().into_inner(),
    ))
    .subscribe(event_bus.clone());

    // 注册用户绑定角色事件
    Arc::new(UserEventListener::new(pool.clone())).subscribe(event_bus.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(register_user_services.clone())
            .app_data(delete_user_services.clone())
            .app_data(update_user_services.clone())
            .app_data(login_services.clone())
            .app_data(create_role_services.clone())
            .app_data(delete_role_services.clone())
            .app_data(update_role_services.clone())
            .app_data(user_query_service.clone())
            .app_data(enforcer.clone())
            .service(
                web::scope("/api/user")
                    .service(register)
                    .service(delete_user)
                    .service(update_user)
                    .service(login)
                    .service(get_login_history),
            )
            .service(
                web::scope("/api/role")
                    .service(create_role)
                    .service(delete_role)
                    .service(update_role),
            )
    })
    .bind(server_url)?
    .run()
    .await
}
