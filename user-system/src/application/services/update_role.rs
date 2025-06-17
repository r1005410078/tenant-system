use crate::application::commands::{
    permission_granted_to_role::{
        PermissionGrantedToRoleCommand, PermissionGrantedToRoleCommandHandler,
    },
    update_role::{UpdateRoleCommand, UpdateRoleCommandHandler},
};

pub struct UpdateRoleService {
    update_role_command_handler: UpdateRoleCommandHandler,
    permission_granted_to_role_command_handler: PermissionGrantedToRoleCommandHandler,
}

impl UpdateRoleService {
    pub fn new(
        update_role_command_handler: UpdateRoleCommandHandler,
        permission_granted_to_role_command_handler: PermissionGrantedToRoleCommandHandler,
    ) -> Self {
        Self {
            update_role_command_handler,
            permission_granted_to_role_command_handler,
        }
    }

    pub async fn execute(&self, command: UpdateRoleCommand) -> anyhow::Result<()> {
        self.update_role_command_handler
            .handle(command.clone())
            .await?;

        // 如果有权限修改就更新
        if let Some(permissions) = command.permissions {
            let command = PermissionGrantedToRoleCommand::new(command.id, permissions);
            self.permission_granted_to_role_command_handler
                .handle(command)
                .await?;
        }

        Ok(())
    }
}
