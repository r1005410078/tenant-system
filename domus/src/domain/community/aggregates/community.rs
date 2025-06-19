use sea_orm::prelude::DateTimeUtc;

use crate::domain::community::{
    events::{
        community_created::CommunityCreatedEvent, community_deleted::CommunityDeletedEvent,
        community_updated::CommunityUpdatedEvent,
    },
    value_objects::{
        community_created_data::CommunityCreateData, community_updated_data::CommunityUpdateData,
    },
};

#[derive(Debug, Clone)]
pub struct CommunityAggregate {
    pub community_id: String,
    pub name: String,
    pub address: String,
    pub deleted_at: Option<DateTimeUtc>,
}

impl CommunityAggregate {
    pub fn new(name: String, address: String) -> CommunityAggregate {
        let community_id = uuid::Uuid::new_v4().to_string();
        CommunityAggregate {
            community_id,
            name,
            address,
            deleted_at: None,
        }
    }

    pub fn create(data: &CommunityCreateData) -> (CommunityAggregate, CommunityCreatedEvent) {
        let aggregate: CommunityAggregate =
            CommunityAggregate::new(data.name.clone(), data.address.clone());

        (aggregate.clone(), data.to_event(&aggregate.community_id))
    }

    pub fn update(&mut self, data: &CommunityUpdateData) -> anyhow::Result<CommunityUpdatedEvent> {
        if self.is_deleted() {
            return Err(anyhow::anyhow!("community is deleted"));
        }

        self.name = data.name.clone().unwrap_or(self.name.clone());
        self.address = data.address.clone().unwrap_or(self.address.clone());

        Ok(data.to_event())
    }

    pub fn delete(&mut self) -> CommunityDeletedEvent {
        self.deleted_at = Some(chrono::Utc::now());
        CommunityDeletedEvent::new(self.community_id.clone())
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}
