use crate::application::commands::create_role::{CreateRoleCommand, CreateRoleCommandHandler};

pub struct CreateRoleService {
    create_role_command_handler: CreateRoleCommandHandler,
}

impl CreateRoleService {
    pub fn new(create_role_command_handler: CreateRoleCommandHandler) -> Self {
        Self {
            create_role_command_handler,
        }
    }

    pub async fn execute(&self, command: CreateRoleCommand) -> anyhow::Result<String> {
        self.create_role_command_handler.handle(command).await
    }
}
