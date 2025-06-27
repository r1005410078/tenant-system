use std::sync::Arc;

use crate::application::commands::delete_user::{DeleteUserCommand, DeleteUserCommandHandler};

pub struct DeleteUserService {
    delete_user_handler: DeleteUserCommandHandler,
}

impl DeleteUserService {
    pub fn new(delete_user_handler: DeleteUserCommandHandler) -> Self {
        Self {
            delete_user_handler,
        }
    }

    pub async fn execute(&self, user_id: String) -> anyhow::Result<()> {
        self.delete_user_handler
            .handle(DeleteUserCommand::new(user_id))
            .await
    }
}
