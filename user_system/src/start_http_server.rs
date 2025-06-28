use sea_orm::DbConn;
use std::{env, sync::Arc};
use user_system::shared::{auth_middleware::AuthMiddleware, casbin::init_casbin::init_casbin};

use crate::{
    application::{
        commands::{
            create_role::CreateRoleCommandHandler, delete_role::DeleteRoleCommandHandler,
            delete_user::DeleteUserCommandHandler, login::LoginCommandHandler,
            permission_granted_to_role::PermissionGrantedToRoleCommandHandler,
            register_user::UserRegistrationHandler, update_role::UpdateRoleCommandHandler,
            update_user::UpdateUserCommandHandler, user_binded_to_roles::UserBindedToRolesHandler,
        },
        listeners::{login::LoginEventListener, role::RoleEventListener, user::UserEventListener},
        queries::{role_query_service::RoleQueryService, user_query_service::UserQueryService},
        services::{
            create_role::CreateRoleService, delete_role::DeleteRoleService,
            delete_user::DeleteUserService, login::LoginService,
            permissions_detail::PermissionsDetailService, register_user::RegisterUserService,
            update_role::UpdateRoleService, update_user::UpdateUserService,
        },
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
    interfaces::controllers::{
        role::{
            create_role, delete_role, detail_role, list_role, permissions_details_list,
            save_permission_detail, update_role,
        },
        user::{delete_user, login, register, update_user},
        user_query::{get_login_history, get_user, get_user_list},
    },
};
use actix_web::{web, App, HttpServer};
use event_bus::{AsyncEventBus, EventListener};

pub async fn run() -> std::io::Result<()> {
    log::init_tracing();
    dotenvy::dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    let pool = create_mysql_pool().await;
    let enforcer = Arc::new(init_casbin().await);

    let event_bus = Arc::new(AsyncEventBus::new(Some(pool.clone())));
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
    let permission_granted_to_role_command_handler = Arc::new(
        PermissionGrantedToRoleCommandHandler::new(role_repo.clone(), event_bus.clone()),
    );
    // 更新角色
    let update_role_services = web::Data::new(UpdateRoleService::new(
        UpdateRoleCommandHandler::new(role_repo.clone(), event_bus.clone()),
        permission_granted_to_role_command_handler.clone(),
    ));

    // 创建角色
    let create_role_services = web::Data::new(CreateRoleService::new(
        CreateRoleCommandHandler::new(role_repo.clone(), event_bus.clone()),
        permission_granted_to_role_command_handler.clone(),
    ));

    // 删除角色
    let delete_role_services = web::Data::new(DeleteRoleService::new(
        DeleteRoleCommandHandler::new(role_repo.clone(), event_bus.clone()),
    ));

    let permissions_detail_service = Arc::new(PermissionsDetailService::new(pool.clone()));
    let auth_middleware = Arc::new(AuthMiddleware::new(enforcer.clone()));

    let QueryService {
        user_query_service,
        role_query_service,
    } = QueryService::register_event_handlers(pool.clone(), event_bus.clone());

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
            .app_data(role_query_service.clone())
            .app_data(permissions_detail_service.clone())
            .service(
                web::scope("/api/user")
                    .wrap(auth_middleware.clone())
                    .service(delete_user)
                    .service(update_user)
                    .service(get_login_history)
                    .service(get_user_list)
                    .service(get_user),
            )
            .service(
                web::scope("/api/role")
                    .wrap(auth_middleware.clone())
                    .service(create_role)
                    .service(delete_role)
                    .service(update_role)
                    .service(detail_role)
                    .service(list_role)
                    .service(permissions_details_list)
                    .service(save_permission_detail),
            )
            .service(web::scope("/api").service(login).service(register))
    })
    .bind(server_url)?
    .run()
    .await
}

struct QueryService {
    user_query_service: web::Data<UserQueryService>,
    role_query_service: web::Data<RoleQueryService>,
}

impl QueryService {
    fn register_event_handlers(pool: Arc<DbConn>, event_bus: Arc<AsyncEventBus>) -> QueryService {
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

        QueryService {
            user_query_service,
            role_query_service,
        }
    }
}
