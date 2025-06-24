use std::sync::Arc;

use crate::application::{
    commands::{create_house::CreateHouseCommand, create_house_handler::CreateHouseCommandHandler},
    listeners::community,
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

    pub async fn execute(&self, mut command: CreateHouseCommand) -> anyhow::Result<()> {
        // 1.创建小区
        // 2.给房屋设置小区ID
        let community_id = self.save_community_service.save(&command.community).await?;
        let mut community = command.community.clone();
        community.id = Some(community_id);
        command.community = community;

        // 1.创建业主
        // 2.给房屋设置业主ID
        let owner_id = self.save_owner_service.save(&command.owner).await?;
        let mut owner = command.owner.clone();
        owner.id = Some(owner_id);
        command.owner = owner;

        // 创建房屋
        self.create_house_command_handler.handle(command).await?;

        Ok(())
    }
}
