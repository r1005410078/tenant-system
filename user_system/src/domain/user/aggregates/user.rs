use crate::domain::{
    security::argon::Argon,
    user::{
        events::{
            login::{LoginEvent, LoginEventFail, LoginEventSuccess},
            user::UserEvent,
            user_binded_to_roles::UserBindedToRolesEvent,
            user_deleted::UserDeletedEvent,
            user_registered::UserRegisteredEvent,
            user_updated::UserUpdatedEvent,
        },
        value_objects::account_status::AccountStatus,
    },
};
use anyhow::Ok;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserAggregate {
    pub id: Uuid,
    // 用户名
    pub username: String,
    // 邮箱
    pub email: Option<String>,
    // 手机号
    pub phone: Option<String>,
    // 密码
    pub password: String,
    // 角色
    pub roles: Vec<String>,
    // 账户状态
    pub account_status: AccountStatus,
    // 注册时间
    pub register_time: DateTime<Utc>,
    // 最后登录时间
    pub last_login_time: Option<DateTime<Utc>>,
    // 删除时间
    pub deleted_at: Option<DateTime<Utc>>,
}

impl UserAggregate {
    // 注册用户
    pub fn register_user(
        username: String,
        email: Option<String>,
        phone: Option<String>,
        password: String,
        roles: Vec<String>,
    ) -> anyhow::Result<(UserAggregate, UserEvent)> {
        let user = UserAggregate {
            id: Uuid::new_v4(),
            username,
            email,
            phone,
            password,
            roles: vec![],
            account_status: AccountStatus::Active,
            register_time: Utc::now(),
            last_login_time: None,
            deleted_at: None,
        };

        user.validate()?;

        let event = UserEvent::UserRegistered(UserRegisteredEvent {
            id: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            phone: user.phone.clone(),
            account_status: user.account_status.to_string(),
            roles,
        });

        Ok((user, event))
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        // 进行用户注册时的验证逻辑
        Ok(())
    }

    // 用户更新
    pub fn update(
        &mut self,
        username: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        password: Option<String>,
        roles: Option<Vec<String>>,
    ) -> UserEvent {
        self.username = username.unwrap_or(self.username.clone());
        self.email = email.or(self.email.clone());
        self.phone = phone.or(self.phone.clone());
        self.password = password
            .map(|p| Argon::password_hash(p.as_str()))
            .unwrap_or(self.password.clone());

        self.roles = roles.unwrap_or(self.roles.clone());

        UserEvent::UserUpdated(UserUpdatedEvent {
            id: self.id.to_string(),
            username: self.username.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            account_status: self.account_status.to_string(),
            roles: Some(self.roles.clone()),
        })
    }

    pub fn login(&mut self, username: &str, password: &str) -> LoginEvent {
        let login_time = Utc::now();
        self.last_login_time = Some(login_time);

        if Argon::verify_password(password, &self.password) {
            LoginEvent::Success(LoginEventSuccess::new(
                self.id.to_string(),
                username.to_string(),
                password.to_string(),
                login_time,
            ))
        } else {
            LoginEvent::Fail(LoginEventFail::new(
                self.id.to_string(),
                username.to_string(),
                password.to_string(),
                login_time,
            ))
        }
    }

    // 删除用户
    pub fn delete(&mut self) -> UserEvent {
        self.deleted_at = Some(Utc::now());
        UserEvent::UserDeleted(UserDeletedEvent::new(self.id.to_string()))
    }

    // 为用户绑定角色
    pub fn bind_roles(&mut self, roles: Vec<String>) -> UserEvent {
        self.roles = roles;
        UserEvent::UserBindedToRoles(UserBindedToRolesEvent {
            user_id: self.id.to_string(),
            roles: self.roles.clone(),
        })
    }
}
