#[derive(Debug, Clone)]
pub struct CommunityDeletedEvent {
    pub id: String,
}

impl CommunityDeletedEvent {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
