use crate::application::commands::delete_owner::{DeleteOwnerCommand, DeleteOwnerCommandHandler};

pub struct DeleteOwnerService {
    delete_owner_command_handler: DeleteOwnerCommandHandler,
}

impl DeleteOwnerService {
    pub fn new(delete_owner_command_handler: DeleteOwnerCommandHandler) -> Self {
        Self {
            delete_owner_command_handler,
        }
    }

    pub async fn execute(&self, command: DeleteOwnerCommand) -> anyhow::Result<()> {
        self.delete_owner_command_handler.handle(command).await
    }
}
