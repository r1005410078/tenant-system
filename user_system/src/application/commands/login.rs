use crate::domain::user::events::login::LoginEvent;
use crate::infrastructure::dtos::user_query_dto::UserQueryDto;
use crate::{
    application::repositories::user_aggreate_repository::UserAggregateRepository,
    domain::user::aggregates::user::UserAggregate,
};
use event_bus::AsyncEventBus;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use user_system::shared::claims::Claims;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfomation {
    pub id: String,
    // 用户名
    pub username: String,
    // 邮箱
    pub email: Option<String>,
    // 手机号
    pub phone: Option<String>,
    // 角色
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfomation {
    token: String,
    user: UserInfomation,
}

impl From<UserAggregate> for UserInfomation {
    fn from(user: UserAggregate) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            phone: user.phone,
            roles: user.roles,
        }
    }
}

impl LoginInfomation {
    pub fn new(token: String, user: UserAggregate) -> Self {
        let user = UserInfomation::from(user);
        Self { token, user }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginCommand {
    pub username: String,
    pub password: String,
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

    pub async fn handle(&self, command: LoginCommand) -> anyhow::Result<LoginInfomation> {
        // 查找用户聚合
        let mut user = self
            .user_repo
            .find_by_username(command.username.as_str())
            .await?;

        println!("user: {:#?}", user);

        let login_event = user.login(command.username.as_str(), command.password.as_str());

        // 保存用户聚合
        self.user_repo.save(&user).await?;

        // 发送事件
        self.event_bus
            .persist_and_publish(login_event.clone())
            .await?;

        if let LoginEvent::Success(_) = login_event {
            // 生成token
            let claims = Claims::new(
                user.id.to_string(),
                user.username.clone(),
                user.roles.clone(),
            );

            // 保存会话 TODO
            Ok(LoginInfomation::new(claims.get_token(), user))
        } else {
            Err(anyhow::anyhow!("用户名或密码错误!"))
        }
    }
}
