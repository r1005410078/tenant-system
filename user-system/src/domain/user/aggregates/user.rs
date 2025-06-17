use crate::domain::{
    password::argon::Argon,
    user::{
        events::{
            login::{LoginEvent, LoginEventFail, LoginEventSuccess},
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
    pub role: Vec<String>,
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
    ) -> anyhow::Result<(UserAggregate, UserRegisteredEvent)> {
        let user = UserAggregate {
            id: Uuid::new_v4(),
            username,
            email,
            phone,
            password,
            role: vec![],
            account_status: AccountStatus::Active,
            register_time: Utc::now(),
            last_login_time: None,
            deleted_at: None,
        };

        user.validate()?;

        let event = UserRegisteredEvent {
            id: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            phone: user.phone.clone(),
            account_status: user.account_status.to_string(),
            role: vec![],
        };

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
    ) -> UserUpdatedEvent {
        self.username = username.unwrap_or(self.username.clone());
        self.email = email.or(self.email.clone());
        self.phone = phone.or(self.phone.clone());
        self.password = Argon::password_hash(&password.unwrap_or(self.password.clone()));

        UserUpdatedEvent {
            id: self.id.to_string().clone(),
            username: self.username.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            account_status: self.account_status.to_string(),
            role: vec![],
        }
    }

    pub fn login(&mut self, username: &str, password: &str) -> LoginEvent {
        let login_time = Utc::now();
        self.last_login_time = Some(login_time);

        if Argon::verify_password(password, &self.password) {
            LoginEvent::Success(LoginEventSuccess::new(
                username.to_string(),
                password.to_string(),
                login_time,
            ))
        } else {
            LoginEvent::Fail(LoginEventFail::new(
                username.to_string(),
                password.to_string(),
                login_time,
            ))
        }
    }

    // 删除用户
    pub fn delete(&mut self) -> UserDeletedEvent {
        self.deleted_at = Some(Utc::now());
        UserDeletedEvent {
            id: self.id.to_string(),
        }
    }

    // 为用户绑定角色
    pub fn bind_roles(&mut self, roles: Vec<String>) -> UserBindedToRolesEvent {
        self.role = roles;
        UserBindedToRolesEvent {
            user_id: self.id.to_string(),
            roles: self.role.clone(),
        }
    }
}
