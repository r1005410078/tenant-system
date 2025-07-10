use crate::domain::community::{
    events::community_deleted::CommunityDeletedEvent, value_objects::commuity::Community,
};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum CommunityEvent {
    Created(Community),
    Updated(Community),
    Deleted(CommunityDeletedEvent),
}
