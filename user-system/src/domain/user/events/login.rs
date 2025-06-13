use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct LoginEventSuccess {
    pub username: String,
    pub password: String,
    pub login_time: DateTime<Utc>,
}

impl LoginEventSuccess {
    pub fn new(username: String, password: String, login_time: DateTime<Utc>) -> Self {
        Self {
            username,
            password,
            login_time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoginEventFail {
    pub username: String,
    pub password: String,
    pub login_time: DateTime<Utc>,
}

impl LoginEventFail {
    pub fn new(username: String, password: String, login_time: DateTime<Utc>) -> Self {
        Self {
            username,
            password,
            login_time,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LoginEvent {
    Success(LoginEventSuccess),
    Fail(LoginEventFail),
}
