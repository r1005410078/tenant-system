use serde::Deserialize;

use crate::domain::community::value_objects::commuity::Community;

#[derive(Debug, Deserialize)]
pub struct SaveCommunityCommand(Community);

impl SaveCommunityCommand {
    pub fn new(community: Community) -> Self {
        SaveCommunityCommand(community)
    }

    pub fn into_inner(&self) -> Community {
        self.0.clone()
    }
}
