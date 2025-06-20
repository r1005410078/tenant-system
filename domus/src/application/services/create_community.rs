use crate::application::commands::{
    create_community::CreateCommunityCommand,
    create_community_handler::CreateCommunityCommandHandler,
};

pub struct CreateCommunityService {
    create_community_handler: CreateCommunityCommandHandler,
}

impl CreateCommunityService {
    pub fn new(create_community_handler: CreateCommunityCommandHandler) -> Self {
        Self {
            create_community_handler,
        }
    }

    pub async fn execute(&self, command: CreateCommunityCommand) -> anyhow::Result<()> {
        self.create_community_handler.handle(command).await
    }
}
