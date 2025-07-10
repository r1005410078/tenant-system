use serde::Serialize;

use crate::domain::owner::{
    events::owner_deleted::OwnerDeletedEvent, value_objects::owner::HouseOwner,
};

#[derive(Debug, Clone, Serialize)]
pub enum OwnerEvent {
    Created(HouseOwner),
    Updated(HouseOwner),
    Deleted(OwnerDeletedEvent),
}
