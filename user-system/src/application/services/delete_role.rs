use crate::application::commands::delete_role::{DeleteRoleCommand, DeleteRoleCommandHandler};

pub struct DeleteRoleService {
    delete_role_command_handler: DeleteRoleCommandHandler,
}

impl DeleteRoleService {
    pub fn new(delete_role_command_handler: DeleteRoleCommandHandler) -> Self {
        Self {
            delete_role_command_handler,
        }
    }

    pub async fn execute(&self, id: String) -> anyhow::Result<()> {
        self.delete_role_command_handler
            .handle(DeleteRoleCommand::new(id))
            .await
    }
}
