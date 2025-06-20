use sea_orm::prelude::DateTimeUtc;

#[derive(Debug, Clone)]
pub struct OwenerDeletedEvent {
    pub owner_id: String,
    pub deleted_at: DateTimeUtc,
}

impl OwenerDeletedEvent {
    pub fn new(owner_id: String, deleted_at: DateTimeUtc) -> Self {
        Self {
            owner_id,
            deleted_at,
        }
    }
}
