use std::sync::Arc;

use event_bus::AsyncEventBus;
use serde::{Deserialize, Serialize};

use crate::{
    application::repositories::user_aggreate_repository::UserAggregateRepository,
    domain::{security::argon::Argon, user::aggregates::user::UserAggregate},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterUserCommand {
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: String,
    pub roles: Option<Vec<String>>,
}

impl RegisterUserCommand {
    #[allow(dead_code)]
    pub fn new(username: String, password: String, roles: Vec<String>) -> Self {
        Self {
            username,
            email: None,
            phone: None,
            password,
            roles: Some(roles),
        }
    }
}

pub struct UserRegistrationHandler {
    event_bus: Arc<AsyncEventBus>,
    user_repo: Arc<dyn UserAggregateRepository>,
}

impl UserRegistrationHandler {
    pub fn new(event_bus: Arc<AsyncEventBus>, user_repo: Arc<dyn UserAggregateRepository>) -> Self {
        UserRegistrationHandler {
            event_bus,
            user_repo,
        }
    }

    pub async fn handle(&self, command: RegisterUserCommand) -> anyhow::Result<UserAggregate> {
        // 用户是否存在
        if self.user_repo.exists(command.username.as_str()).await? {
            return Err(anyhow::anyhow!("用户名不可用"));
        }

        let (user, event) = UserAggregate::register_user(
            command.username,
            command.email,
            command.phone,
            Argon::password_hash(&command.password),
            command.roles.unwrap_or(vec![]),
        )?;

        // 创建新的用户聚合
        self.user_repo.create(&user).await?;

        // 发布事件到事件总线（简化的逻辑）
        self.event_bus.persist_and_publish(event).await?;

        Ok(user)
    }
}
