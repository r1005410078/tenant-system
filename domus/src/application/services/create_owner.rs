use crate::application::commands::create_owner::{CreateOwnerCommand, CreateOwnerCommandHandler};

pub struct CreateOwnerService {
    create_owner_command_handler: CreateOwnerCommandHandler,
}

impl CreateOwnerService {
    pub fn new(create_owner_command_handler: CreateOwnerCommandHandler) -> Self {
        Self {
            create_owner_command_handler,
        }
    }

    pub async fn execute(&self, command: CreateOwnerCommand) -> anyhow::Result<()> {
        self.create_owner_command_handler.handle(command).await
    }
}
