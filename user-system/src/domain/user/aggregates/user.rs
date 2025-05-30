use crate::{
    application::repositories::user,
    domain::{
        self,
        password::argon::Argon,
        user::{
            events::{
                login::{LoginEvent, LoginEventFail, LoginEventSuccess},
                user_registered::UserRegisteredEvent,
            },
            value_objects::account_status::AccountStatus,
        },
    },
};
use anyhow::Ok;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
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
    pub role: String,
    // 账户状态
    pub account_status: AccountStatus,
    // 注册时间
    pub register_time: DateTime<Utc>,
    // 最后登录时间
    pub last_login_time: Option<DateTime<Utc>>,
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
            role: "user".to_string(),
            account_status: AccountStatus::Active,
            register_time: Utc::now(),
            last_login_time: None,
        };

        user.validate()?;

        let event = UserRegisteredEvent {
            id: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            phone: user.phone.clone(),
            account_status: user.account_status.to_string(),
            role: user.role.clone(),
        };

        Ok((user, event))
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        // 进行用户注册时的验证逻辑
        Ok(())
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
}
