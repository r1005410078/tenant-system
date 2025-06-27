use std::sync::Arc;

use crate::application::commands::{
    create_role::{CreateRoleCommand, CreateRoleCommandHandler},
    permission_granted_to_role::{
        PermissionGrantedToRoleCommand, PermissionGrantedToRoleCommandHandler,
    },
};

pub struct CreateRoleService {
    create_role_command_handler: CreateRoleCommandHandler,
    permission_granted_to_role_command_handler: Arc<PermissionGrantedToRoleCommandHandler>,
}

impl CreateRoleService {
    pub fn new(
        create_role_command_handler: CreateRoleCommandHandler,
        permission_granted_to_role_command_handler: Arc<PermissionGrantedToRoleCommandHandler>,
    ) -> Self {
        Self {
            create_role_command_handler,
            permission_granted_to_role_command_handler,
        }
    }

    pub async fn execute(&self, command: CreateRoleCommand) -> anyhow::Result<String> {
        let role_id = self
            .create_role_command_handler
            .handle(command.clone())
            .await?;

        // 如果有权限修改就更新
        if let Some(permissions) = command.permissions {
            let command = PermissionGrantedToRoleCommand::new(role_id.clone(), permissions.clone());
            self.permission_granted_to_role_command_handler
                .handle(command)
                .await?;
        }

        Ok(role_id)
    }
}
