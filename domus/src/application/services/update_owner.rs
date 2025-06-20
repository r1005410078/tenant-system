use std::sync::Arc;

use crate::application::commands::update_owner::{UpdateOwnerCommand, UpdateOwnerCommandHandler};

pub struct UpdateOwnerService {
    update_owner_command_handler: Arc<UpdateOwnerCommandHandler>,
}
impl UpdateOwnerService {
    pub fn new(update_owner_command_handler: Arc<UpdateOwnerCommandHandler>) -> Self {
        Self {
            update_owner_command_handler,
        }
    }

    pub async fn execute(&self, command: UpdateOwnerCommand) -> anyhow::Result<()> {
        self.update_owner_command_handler.handle(command).await
    }
}
