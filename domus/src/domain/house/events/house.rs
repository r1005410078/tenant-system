use serde::Serialize;

use crate::domain::house::{
    events::{
        house_deleted::HouseDeletedEvent, house_published::HousePublishedEvent,
        house_unpublished::HouseUnpublishedEvent,
    },
    value_objects::house::House,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub enum HouseEvent {
    Created(House),
    Updated(House),
    Published(HousePublishedEvent),
    Unpublished(HouseUnpublishedEvent),
    Deleted(HouseDeletedEvent),
}
