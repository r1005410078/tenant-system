use std::sync::Arc;

use crate::application::commands::{
    update_community::UpdateCommunityCommand,
    update_community_handler::UpdateCommunityCommandHandler,
    update_house::UpdateHouseCommand,
    update_house_handler::UpdateHouseCommandHandler,
    update_owner::{UpdateOwenerCommand, UpdateOwnerCommandHandler},
};

pub struct UpdateHouseService {
    update_house_command_handler: UpdateHouseCommandHandler,
    update_community_command_handler: Arc<UpdateCommunityCommandHandler>,
    update_owner_command_handler: Arc<UpdateOwnerCommandHandler>,
}

impl UpdateHouseService {
    pub fn new(
        update_house_command_handler: UpdateHouseCommandHandler,
        update_community_command_handler: Arc<UpdateCommunityCommandHandler>,
        update_owner_command_handler: Arc<UpdateOwnerCommandHandler>,
    ) -> Self {
        Self {
            update_house_command_handler,
            update_community_command_handler,
            update_owner_command_handler,
        }
    }

    pub async fn execute(&self, command: UpdateHouseCommand) -> anyhow::Result<()> {
        if let Some(ref community) = command.community {
            let command = UpdateCommunityCommand {
                // 小区ID
                community_id: community.get_id()?,
                // 小区名称
                name: community.name.clone(),
                // 小区地址
                address: community.address.clone(),
                // 城市
                city: community.city.clone(),
                // 小区年限
                year_built: community.year_built.clone(),
                // 小区类型
                community_type: community.community_type.clone(),
                // 小区描述
                description: community.description.clone(),
                // 小区图片
                image: community.image.clone(),
                // 位置
                location: community.location.clone(),
            };

            self.update_community_command_handler
                .handle(command)
                .await?;
        }

        if let Some(ref owner) = command.owner {
            let command = UpdateOwenerCommand {
                id: owner.get_id()?,
                name: owner.name.clone(),
                phone: owner.phone.clone(),
                id_card: owner.id_card.clone(),
                id_card_images: owner.id_card_images.clone(),
                description: owner.description.clone(),
            };

            self.update_owner_command_handler.handle(command).await?;
        }

        self.update_house_command_handler.handle(command).await?;

        Ok(())
    }
}
