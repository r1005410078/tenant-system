use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum AccountStatus {
    // 激活
    Active,
    // 未激活
    Inactive,
    // 锁定
    Locked,
}

impl Default for AccountStatus {
    fn default() -> Self {
        AccountStatus::Active
    }
}

impl AccountStatus {
    pub fn to_string(&self) -> String {
        match self {
            AccountStatus::Active => "active".to_string(),
            AccountStatus::Inactive => "inactive".to_string(),
            AccountStatus::Locked => "locked".to_string(),
        }
    }

    pub fn from_string(status: &str) -> AccountStatus {
        match status {
            "active" => AccountStatus::Active,
            "inactive" => AccountStatus::Inactive,
            "locked" => AccountStatus::Locked,
            _ => AccountStatus::default(),
        }
    }
}

// 字符串可以into 成 AccountStatus
impl From<String> for AccountStatus {
    fn from(status: String) -> Self {
        AccountStatus::from_string(&status)
    }
}
