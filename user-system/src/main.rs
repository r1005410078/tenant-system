mod application;
mod domain;
mod infrastructure;
use application::{
    commands::register_user::{RegisterUserCommand, UserRegistrationHandler},
    listeners::user_event::UserEventListener,
    services::register_user::RegisterUserService,
};
use event_bus::AsyncEventBus;
use infrastructure::user::user_aggregate_repository::MySqlUserAggregateRepository;
use sea_orm::*;
use std::{env, sync::Arc};
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone)]
pub struct OrderPlacedEvent {
    pub order_id: String,
    pub amount: f32,
}

#[tokio::main]
async fn main() {
    log::init_tracing();

    info!("用户注册成功");
    debug!(user_id = 42, "调试用户信息");
    warn!(target: "auth", "授权失败");
    error!(error = %"invalid_token", "请求失败");

    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // establish connection to database and apply migrations
    // -> create post table if not exists
    let conn = Arc::new(Database::connect(&db_url).await.unwrap());
    // Migrator::up(&conn, None).await.unwrap();

    let event_bus = Arc::new(AsyncEventBus::new());

    let mysql_user_aggregate_repository = Arc::new(MySqlUserAggregateRepository::new(conn.clone()));

    let handler = Arc::new(UserRegistrationHandler::new(
        event_bus.clone(),
        mysql_user_aggregate_repository,
    ));

    UserEventListener::new(event_bus.clone(), conn.clone())
        .start_listening()
        .await;

    if let Err(err) = RegisterUserService::new(handler.clone())
        .execute(RegisterUserCommand {
            username: "zhangsan41".to_string(),
            email: None,
            phone: None,
            password: "123456".to_string(),
        })
        .await
    {
        println!("Failed to register user: {}", err);
    }
}
