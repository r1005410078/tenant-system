use std::sync::Arc;

use crate::application::{
    commands::{
        update_community::UpdateCommunityCommand,
        update_community_handler::UpdateCommunityCommandHandler,
        update_house::UpdateHouseCommand,
        update_house_handler::UpdateHouseCommandHandler,
        update_owner::{UpdateOwnerCommand, UpdateOwnerCommandHandler},
    },
    services::{save_community::SaveCommunityService, save_owner::SaveOwnerService},
};

pub struct UpdateHouseService {
    update_house_command_handler: UpdateHouseCommandHandler,
    save_community_service: Arc<SaveCommunityService>,
    save_owner_service: Arc<SaveOwnerService>,
}

impl UpdateHouseService {
    pub fn new(
        update_house_command_handler: UpdateHouseCommandHandler,
        save_community_service: Arc<SaveCommunityService>,
        save_owner_service: Arc<SaveOwnerService>,
    ) -> Self {
        Self {
            update_house_command_handler,
            save_community_service,
            save_owner_service,
        }
    }

    pub async fn execute(&self, command: UpdateHouseCommand) -> anyhow::Result<()> {
        if let Some(ref community) = command.community {
            self.save_community_service.save(community).await?;
        }

        if let Some(ref owner) = command.owner {
            self.save_owner_service.save(owner).await?;
        }

        self.update_house_command_handler.handle(command).await?;

        Ok(())
    }
}
