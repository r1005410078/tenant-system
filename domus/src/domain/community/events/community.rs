use serde::Serialize;

use crate::domain::community::events::{
    community_created::CommunityCreatedEvent, community_deleted::CommunityDeletedEvent,
    community_updated::CommunityUpdatedEvent,
};

#[derive(Debug, Clone, Serialize)]
pub enum CommunityEvent {
    Created(CommunityCreatedEvent),
    Updated(CommunityUpdatedEvent),
    Deleted(CommunityDeletedEvent),
}
