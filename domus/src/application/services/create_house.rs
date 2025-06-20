use std::sync::Arc;

use crate::application::commands::{
    create_community::CreateCommunityCommand,
    create_community_handler::CreateCommunityCommandHandler,
    create_house::CreateHouseCommand,
    create_house_handler::CreateHouseCommandHandler,
    create_owner::{CreateOwnerCommand, CreateOwnerCommandHandler},
};

pub struct CreateHouseService {
    create_house_command_handler: CreateHouseCommandHandler,
    create_community_command_handler: Arc<CreateCommunityCommandHandler>,
    create_owner_command_handler: Arc<CreateOwnerCommandHandler>,
}

impl CreateHouseService {
    pub fn new(
        create_house_command_handler: CreateHouseCommandHandler,
        create_community_command_handler: Arc<CreateCommunityCommandHandler>,
        create_owner_command_handler: Arc<CreateOwnerCommandHandler>,
    ) -> Self {
        Self {
            create_house_command_handler,
            create_community_command_handler,
            create_owner_command_handler,
        }
    }

    pub async fn execute(&self, command: CreateHouseCommand) -> anyhow::Result<()> {
        // 创建小区
        if let Some(ref community) = command.community {
            let command = CreateCommunityCommand {
                // 小区名称
                name: community.get_name()?,
                // 小区地址
                address: community.get_address()?,
                // 城市
                city: community.get_city()?,
                // 小区年限
                year_built: community.get_year_built()?,
                // 小区类型
                community_type: community.get_community_type()?,
                // 小区描述
                description: community.description.clone(),
                // 小区图片
                image: community.image.clone(),
                // 位置
                location: community.location.clone(),
            };

            self.create_community_command_handler
                .handle(command)
                .await?;
        }

        // 创建业主
        if let Some(ref owner) = command.owner {
            let command = CreateOwnerCommand {
                // 业主名称
                name: owner.get_name()?,
                // 业主身份证号
                id_card: owner.id_card.clone(),
                // 业主电话
                phone: owner.get_phone()?,
                // 业主身份证照片
                id_card_images: owner.id_card_images.clone(),
                // 业主描述
                description: owner.description.clone(),
            };

            self.create_owner_command_handler.handle(command).await?;
        }

        // 创建房屋
        self.create_house_command_handler.handle(command).await?;

        Ok(())
    }
}
