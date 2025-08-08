use std::sync::Arc;

use crate::{
    application::{
        commands::{save_house::SaveHouseCommand, save_house_handler::SaveHouseCommandHandler},
        services::{save_community::SaveCommunityService, save_owner::SaveOwnerService},
    },
    domain::house::value_objects::house::{House, HouseData},
};

pub struct SaveHouseService {
    pub save_house_command_handler: SaveHouseCommandHandler,
    pub save_community_service: Arc<SaveCommunityService>,
    pub save_owner_service: Arc<SaveOwnerService>,
}

impl SaveHouseService {
    pub fn new(
        save_house_command_handler: SaveHouseCommandHandler,
        save_community_service: Arc<SaveCommunityService>,
        save_owner_service: Arc<SaveOwnerService>,
    ) -> Self {
        Self {
            save_house_command_handler,
            save_community_service,
            save_owner_service,
        }
    }

    pub async fn execute(&self, house_data: HouseData) -> anyhow::Result<House> {
        if house_data.house.is_none() {
            return Err(anyhow::anyhow!("参数不完整"));
        }

        let mut house = house_data.house.unwrap();
        // 1.保存小区
        if let Some(community) = house_data.community {
            let community_id = self.save_community_service.execute(community).await?;
            house.community_id = Some(community_id);
        }

        // 1.保存业主
        if let Some(owner) = house_data.owner {
            let owner_id = self.save_owner_service.execute(owner).await?;
            house.owner_id = Some(owner_id);
        }

        // 创建房屋
        let house_id = self
            .save_house_command_handler
            .handle(SaveHouseCommand::new(house.clone()))
            .await?;

        house.id = Some(house_id.clone());

        Ok(house)
    }
}
