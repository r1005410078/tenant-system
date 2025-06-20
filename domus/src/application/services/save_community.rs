use std::sync::Arc;

use crate::{
    application::commands::{
        create_community::CreateCommunityCommand,
        create_community_handler::CreateCommunityCommandHandler,
        update_community::UpdateCommunityCommand,
        update_community_handler::UpdateCommunityCommandHandler,
    },
    domain::house::value_objects::house::Community,
};

pub struct SaveCommunityService {
    pub create_community_command_handler: Arc<CreateCommunityCommandHandler>,
    pub update_community_command_handler: Arc<UpdateCommunityCommandHandler>,
}

impl SaveCommunityService {
    pub fn new(
        create_community_command_handler: Arc<CreateCommunityCommandHandler>,
        update_community_command_handler: Arc<UpdateCommunityCommandHandler>,
    ) -> Self {
        Self {
            create_community_command_handler,
            update_community_command_handler,
        }
    }

    pub async fn save(&self, community: &Community) -> anyhow::Result<()> {
        if community.id.is_some() {
            // 如果小区ID存在，则更新小区信息
            let command = UpdateCommunityCommand::from(community);
            self.update_community_command_handler
                .handle(command)
                .await?;
        } else {
            // 创建小区
            let command = CreateCommunityCommand::from(community)?;
            self.create_community_command_handler
                .handle(command)
                .await?;
        }

        Ok(())
    }
}
