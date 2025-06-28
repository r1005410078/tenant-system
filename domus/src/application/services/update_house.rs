use std::sync::Arc;

use crate::application::{
    commands::{update_house::UpdateHouseCommand, update_house_handler::UpdateHouseCommandHandler},
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

    pub async fn execute(&self, mut command: UpdateHouseCommand) -> anyhow::Result<()> {
        if let Some(ref community) = command.community {
            // 有更新小区
            let community_id = self.save_community_service.save(community).await?;
            if community.id.is_none() {
                // 如果小区ID不存在，则更新小区信息
                let mut new_community = community.clone();
                new_community.id = Some(community_id);
                command.community = Some(new_community);
            }
        }

        if let Some(ref owner) = command.owner {
            let owner_id = self.save_owner_service.save(owner).await?;
            if owner.id.is_none() {
                // 如果业主ID不存在，则更新业主信息
                let mut new_owner = owner.clone();
                new_owner.id = Some(owner_id);
                command.owner = Some(new_owner);
            }
        }

        self.update_house_command_handler.handle(command).await?;

        Ok(())
    }
}
