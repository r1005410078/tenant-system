use serde::Serialize;

use crate::domain::house::events::{
    house_created::HouseCreatedEvent, house_deleted::HouseDeletedEvent,
    house_published::HousePublishedEvent, house_unpublished::HouseUnpublishedEvent,
    house_updated::HouseUpdatedEvent,
};

#[derive(Debug, Clone, Serialize)]
pub enum HouseEvent {
    Created(HouseCreatedEvent),
    Updated(HouseUpdatedEvent),
    Published(HousePublishedEvent),
    Unpublished(HouseUnpublishedEvent),
    Deleted(HouseDeletedEvent),
}
