use std::sync::Arc;

use crate::{
    application::commands::{
        save_owner::SaveOwnerCommand, save_owner_handler::SaveOwnerCommandHandler,
    },
    domain::owner::value_objects::owner::HouseOwner,
};

pub struct SaveOwnerService {
    pub save_owner_command_handler: Arc<SaveOwnerCommandHandler>,
}

impl SaveOwnerService {
    pub fn new(save_owner_command_handler: Arc<SaveOwnerCommandHandler>) -> Self {
        Self {
            save_owner_command_handler,
        }
    }

    pub async fn execute(&self, owner: HouseOwner) -> anyhow::Result<String> {
        self.save_owner_command_handler
            .handle(SaveOwnerCommand::new(owner))
            .await
    }
}
