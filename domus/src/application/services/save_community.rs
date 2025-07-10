use crate::{
    application::commands::{
        save_community::SaveCommunityCommand, save_community_handler::SaveCommunityCommandHandler,
    },
    domain::community::value_objects::commuity::Community,
};
use std::sync::Arc;

pub struct SaveCommunityService {
    pub save_community_command_handler: Arc<SaveCommunityCommandHandler>,
}

impl SaveCommunityService {
    pub fn new(save_community_command_handler: Arc<SaveCommunityCommandHandler>) -> Self {
        Self {
            save_community_command_handler,
        }
    }

    pub async fn execute(&self, community: Community) -> anyhow::Result<String> {
        self.save_community_command_handler
            .handle(SaveCommunityCommand::new(community))
            .await
    }
}
