use crate::application::repositories::user_aggreate_repository::UserAggregateRepository;
use crate::application::services::claims::Claims;
use crate::domain::user::events::login::LoginEvent;
use event_bus::AsyncEventBus;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct LoginCommand {
    pub username: String,
    pub password: String,
}

impl LoginCommand {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

pub struct LoginCommandHandler {
    user_repo: Arc<dyn UserAggregateRepository>,
    event_bus: Arc<AsyncEventBus>,
}

impl LoginCommandHandler {
    pub fn new(user_repo: Arc<dyn UserAggregateRepository>, event_bus: Arc<AsyncEventBus>) -> Self {
        Self {
            user_repo,
            event_bus,
        }
    }

    pub async fn handle(&self, command: LoginCommand) -> anyhow::Result<String> {
        // 查找用户聚合
        let mut user = self
            .user_repo
            .find_by_username(command.username.as_str())
            .await
            .ok_or(anyhow::anyhow!("密码错误或账号不存在"))?;

        let login_event = user.login(command.username.as_str(), command.password.as_str());

        // 保存用户聚合
        self.user_repo.save(&user).await?;

        // 发送事件
        self.event_bus.publish(login_event.clone()).await;

        if let LoginEvent::Success(_) = login_event {
            // 生成token
            let claims = Claims::new(
                user.id.to_string(),
                user.username.clone(),
                user.roles.clone(),
            );

            // 保存会话 TODO
            Ok(claims.get_token())
        } else {
            Err(anyhow::anyhow!("密码错误!"))
        }
    }
}
