use std::sync::Arc;

use crate::application::{
    commands::{create_house::CreateHouseCommand, create_house_handler::CreateHouseCommandHandler},
    services::{save_community::SaveCommunityService, save_owner::SaveOwnerService},
};

pub struct CreateHouseService {
    pub create_house_command_handler: CreateHouseCommandHandler,
    pub save_community_service: Arc<SaveCommunityService>,
    pub save_owner_service: Arc<SaveOwnerService>,
}

impl CreateHouseService {
    pub fn new(
        create_house_command_handler: CreateHouseCommandHandler,
        save_community_service: Arc<SaveCommunityService>,
        save_owner_service: Arc<SaveOwnerService>,
    ) -> Self {
        Self {
            create_house_command_handler,
            save_community_service,
            save_owner_service,
        }
    }

    pub async fn execute(&self, command: CreateHouseCommand) -> anyhow::Result<()> {
        // 创建小区
        if let Some(ref community) = command.community {
            self.save_community_service.save(community).await?;
        }

        // 创建业主
        if let Some(ref owner) = command.owner {
            self.save_owner_service.save(owner).await?;
        }

        // 创建房屋
        self.create_house_command_handler.handle(command).await?;

        Ok(())
    }
}
