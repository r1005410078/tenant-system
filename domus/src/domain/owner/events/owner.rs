use serde::Serialize;

use crate::domain::owner::events::{
    owner_created::OwnerCreatedEvent, owner_deleted::OwnerDeletedEvent,
    owner_updated::OwnerUpdatedEvent,
};

#[derive(Debug, Clone, Serialize)]
pub enum OwnerEvent {
    Created(OwnerCreatedEvent),
    Updated(OwnerUpdatedEvent),
    Deleted(OwnerDeletedEvent),
}
