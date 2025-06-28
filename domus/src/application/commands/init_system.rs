use std::env;

use clap::{command, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Parser)]
#[command(name = "house-system")]
#[command(about = "房源管理系统 CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<InitSystemCommand>,
}

#[derive(Subcommand)]
pub enum InitSystemCommand {
    InitSystem,
}

pub struct InitSystemCommandHandler {}

impl InitSystemCommandHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn handle(&self) -> anyhow::Result<()> {
        let port = env::var("USER_SYSTEM_PORT").unwrap_or("9001".to_string());
        let url = format!("http://127.0.0.1:{}/api", port);

        // 登陆
        let login_body = json!({"username": "admin", "password": "admin"});

        let res = reqwest::Client::new()
            .post(format!("{}/login", url))
            .json(&login_body)
            .send()
            .await?;

        if !res.status().is_success() {
            println!("登陆失败 {}", res.status());
            return Ok(());
        }

        let login_response: LoginResponse = res.json().await?;
        let token = login_response.data.token.clone();
        let role_id = login_response.data.user.roles.first().unwrap().clone();

        // 创建权限
        let permissions_body = json!([
            {
                "name": "房源管理",
                "source": "^/api/domus/management/*",
                "action": "POST",
                "description": "管理房源的增加，删除，修改"
            },
            {
                "name": "房源查询",
                "source": "^/api/domus/query/*",
                "action": "GET",
                "description": "房源查询"
            }
        ]);

        let res = reqwest::Client::new()
            .post(format!("{}/role/permissions_details/save", url))
            .json(&permissions_body)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if !res.status().is_success() {
            println!("权限数据初始化失败 {}", res.status());
        }

        // 获取所有的权限
        let res = reqwest::Client::new()
            .get(format!("{}/role/permissions_details/list", url))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if !res.status().is_success() {
            println!("权限数据初始化失败 {}", res.status());
            return Ok(());
        }

        let permissions_details: PermissionsDetailsResponse = res.json().await?;

        let mut permissions: Vec<Permission> = vec![];
        for permission in permissions_details.data {
            permissions.push(Permission {
                resouce: permission.source,
                permission: permission.action,
            });
        }

        // 绑定所有权限
        let res = reqwest::Client::new()
            .post(format!("{}/role/update", url))
            .json(&json!({"id": role_id, "permissions": permissions}))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if !res.status().is_success() {
            println!("权限数据初始化失败 {}", res.status());
            return Ok(());
        }

        println!("权限数据初始化成功");

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub code: i32,
    pub data: LoginInfomation,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginInfomation {
    pub token: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub roles: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionsDetailsResponse {
    pub code: i32,
    pub data: Vec<PermissionsDetails>,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionsDetails {
    id: String,
    source: String,
    action: String,
    name: String,
    description: String,
    created_at: String,
    updated_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Permission {
    // 资源
    pub resouce: String,
    // 权限
    pub permission: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn init_system() {
        let handler = InitSystemCommandHandler::new();
        if let Err(e) = handler.handle().await {
            println!("初始化失败 {}", e);
        }
    }
}
