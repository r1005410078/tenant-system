use std::{sync::Arc, vec};

use clap::{arg, command, Parser, Subcommand};
use sea_orm::{ActiveValue::Set, DbConn, EntityTrait};

use crate::{
    application::{
        commands::{create_role::CreateRoleCommand, register_user::RegisterUserCommand},
        services::{create_role::CreateRoleService, register_user::RegisterUserService},
    },
    domain::roles::events::permission_granted_to_role::Permission,
    infrastructure::entitiy::permissions_detail,
};

#[derive(Parser)]
#[command(name = "house-system")]
#[command(about = "房源管理系统 CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<InitSystemCommand>,
}

#[derive(Subcommand)]
pub enum InitSystemCommand {
    InitSystem {
        #[arg(long, default_value = "admin")]
        admin_name: String,
        #[arg(long, default_value = "admin")]
        admin_password: String,
    },
}

pub struct InitSystemCommandHandler {
    pool: Arc<DbConn>,
    register_user_service: RegisterUserService,
    create_role_service: CreateRoleService,
}

impl InitSystemCommandHandler {
    pub fn new(
        pool: Arc<DbConn>,
        register_user_service: RegisterUserService,
        create_role_service: CreateRoleService,
    ) -> Self {
        Self {
            pool,
            register_user_service,
            create_role_service,
        }
    }

    pub async fn handle(&self, username: String, password: String) -> anyhow::Result<()> {
        self.add_default_permissions().await?;

        println!("获取所有权限");
        // 获取所有权限
        let permissions: Vec<Permission> = permissions_detail::Entity::find()
            .all(self.pool.as_ref())
            .await?
            .iter()
            .map(|model| Permission {
                source: model.source.clone(),
                action: model.action.clone(),
            })
            .collect();

        println!("创建角色");
        // 创建角色
        let role_id = self
            .create_role_service
            .execute(CreateRoleCommand {
                name: "admin".to_string(),
                description: Some("超级管理员".to_string()),
                permissions: Some(permissions),
            })
            .await?;

        println!("创建超级管理员");
        // 创建超级管理员
        let roles = Some(vec![role_id]);
        self.register_user_service
            .execute(RegisterUserCommand {
                username,
                password,
                roles,
                email: None,
                phone: None,
            })
            .await?;

        println!("初始化完成");

        Ok(())
    }

    // 创建有哪些权限
    async fn add_default_permissions(&self) -> anyhow::Result<()> {
        // 1. 添加一些权限
        if permissions_detail::Entity::find()
            .all(self.pool.as_ref())
            .await?
            .len()
            == 0
        {
            let permissions = vec![
                permissions_detail::ActiveModel {
                    id: Set(uuid::Uuid::new_v4().to_string()),
                    name: Set("角色管理".to_string()),
                    source: Set("^/api/user_system/role/*".to_string()),
                    action: Set("POST".to_string()),
                    description: Set(Some("管理角色的增加，删除，修改".to_string())),
                    ..Default::default()
                },
                permissions_detail::ActiveModel {
                    id: Set(uuid::Uuid::new_v4().to_string()),
                    name: Set("获取角色信息".to_string()),
                    source: Set("^/api/user_system/role/*".to_string()),
                    action: Set("GET".to_string()),
                    description: Set(Some("获取角色信息".to_string())),
                    ..Default::default()
                },
                permissions_detail::ActiveModel {
                    id: Set(uuid::Uuid::new_v4().to_string()),
                    name: Set("用户管理".to_string()),
                    source: Set("^/api/user_system/user/*".to_string()),
                    action: Set("POST".to_string()),
                    description: Set(Some("管理用户的增加，删除，修改".to_string())),
                    ..Default::default()
                },
                permissions_detail::ActiveModel {
                    id: Set(uuid::Uuid::new_v4().to_string()),
                    name: Set("获取用户管信息".to_string()),
                    source: Set("^/api/user_system/user/*".to_string()),
                    action: Set("GET".to_string()),
                    description: Set(Some("获取用户信息".to_string())),
                    ..Default::default()
                },
            ];

            permissions_detail::Entity::insert_many(permissions)
                .exec(self.pool.as_ref())
                .await?;
        }

        Ok(())
    }
}
