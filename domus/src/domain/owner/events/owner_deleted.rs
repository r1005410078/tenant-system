use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct OwnerDeletedEvent {
    pub owner_id: String,
    pub deleted_at: DateTimeUtc,
}

impl OwnerDeletedEvent {
    pub fn new(owner_id: String, deleted_at: DateTimeUtc) -> Self {
        Self {
            owner_id,
            deleted_at,
        }
    }
}
