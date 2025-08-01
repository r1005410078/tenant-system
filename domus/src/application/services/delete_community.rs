use crate::application::commands::delete_community::{
    DeleteCommunityCommand, DeleteCommunityCommandHandler,
};

pub struct DeleteCommunityService {
    delete_community_handler: DeleteCommunityCommandHandler,
}

impl DeleteCommunityService {
    pub fn new(delete_community_handler: DeleteCommunityCommandHandler) -> Self {
        Self {
            delete_community_handler,
        }
    }

    pub async fn execute(&self, community_id: String) -> anyhow::Result<()> {
        self.delete_community_handler
            .handle(DeleteCommunityCommand::new(community_id))
            .await
    }
}
