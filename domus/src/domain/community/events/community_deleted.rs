use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CommunityDeletedEvent {
    pub id: String,
}

impl CommunityDeletedEvent {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
