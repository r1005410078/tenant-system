use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LoginEventSuccess {
    pub user_id: String,
    pub username: String,
    pub password: String,
    pub login_time: DateTime<Utc>,
}

impl LoginEventSuccess {
    pub fn new(
        user_id: String,
        username: String,
        password: String,
        login_time: DateTime<Utc>,
    ) -> Self {
        Self {
            user_id,
            username,
            password,
            login_time,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginEventFail {
    pub user_id: String,
    pub username: String,
    pub password: String,
    pub login_time: DateTime<Utc>,
}

impl LoginEventFail {
    pub fn new(
        user_id: String,
        username: String,
        password: String,
        login_time: DateTime<Utc>,
    ) -> Self {
        Self {
            user_id,
            username,
            password,
            login_time,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum LoginEvent {
    Success(LoginEventSuccess),
    Fail(LoginEventFail),
}
