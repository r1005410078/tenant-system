use crate::application::commands::{
    update_community::UpdateCommunityCommand,
    update_community_handler::UpdateCommunityCommandHandler,
};

pub struct UpdateCommunityService {
    update_community_handler: UpdateCommunityCommandHandler,
}

impl UpdateCommunityService {
    pub fn new(update_community_handler: UpdateCommunityCommandHandler) -> Self {
        Self {
            update_community_handler,
        }
    }

    pub async fn execute(&self, command: UpdateCommunityCommand) -> anyhow::Result<()> {
        self.update_community_handler.handle(command).await
    }
}
