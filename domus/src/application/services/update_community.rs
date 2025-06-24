use std::sync::Arc;

use crate::application::commands::{
    update_community::UpdateCommunityCommand,
    update_community_handler::UpdateCommunityCommandHandler,
};

pub struct UpdateCommunityService {
    update_community_handler: Arc<UpdateCommunityCommandHandler>,
}

impl UpdateCommunityService {
    pub fn new(update_community_handler: Arc<UpdateCommunityCommandHandler>) -> Self {
        Self {
            update_community_handler,
        }
    }

    pub async fn execute(&self, command: UpdateCommunityCommand) -> anyhow::Result<String> {
        self.update_community_handler.handle(command).await
    }
}
