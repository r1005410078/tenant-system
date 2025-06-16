use crate::application::commands::update_role::{UpdateRoleCommand, UpdateRoleCommandHandler};

pub struct UpdateRoleService {
    update_role_command_handler: UpdateRoleCommandHandler,
}

impl UpdateRoleService {
    pub fn new(update_role_command_handler: UpdateRoleCommandHandler) -> Self {
        Self {
            update_role_command_handler,
        }
    }

    pub async fn execute(&self, command: UpdateRoleCommand) -> anyhow::Result<()> {
        self.update_role_command_handler.handle(command).await
    }
}
