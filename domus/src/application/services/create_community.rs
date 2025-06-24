use std::sync::Arc;

use crate::application::commands::{
    create_community::CreateCommunityCommand,
    create_community_handler::CreateCommunityCommandHandler,
};

pub struct CreateCommunityService {
    create_community_handler: Arc<CreateCommunityCommandHandler>,
}

impl CreateCommunityService {
    pub fn new(create_community_handler: Arc<CreateCommunityCommandHandler>) -> Self {
        Self {
            create_community_handler,
        }
    }

    pub async fn execute(&self, command: CreateCommunityCommand) -> anyhow::Result<String> {
        self.create_community_handler.handle(command).await
    }
}
