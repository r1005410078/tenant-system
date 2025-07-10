use sea_orm::prelude::DateTimeUtc;

use crate::domain::community::{
    events::{community::CommunityEvent, community_deleted::CommunityDeletedEvent},
    value_objects::commuity::Community,
};

#[derive(Debug, Clone)]
pub struct CommunityAggregate {
    pub community_id: String,
    pub name: String,
    pub address: String,
    pub deleted_at: Option<DateTimeUtc>,
}

impl CommunityAggregate {
    pub fn new(name: String, address: String, id: Option<String>) -> CommunityAggregate {
        let community_id = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        CommunityAggregate {
            community_id,
            name,
            address,
            deleted_at: None,
        }
    }

    pub fn create(data: &Community) -> anyhow::Result<(CommunityAggregate, CommunityEvent)> {
        data.validate()?;

        let aggregate: CommunityAggregate = CommunityAggregate::new(
            data.name.clone().unwrap().clone(),
            data.address.clone().unwrap().clone(),
            data.id.clone(),
        );

        Ok((aggregate.clone(), CommunityEvent::Created(data.clone())))
    }

    pub fn update(&mut self, data: &Community) -> anyhow::Result<CommunityEvent> {
        if self.is_deleted() {
            return Err(anyhow::anyhow!("community is deleted"));
        }

        self.name = data.name.clone().unwrap_or(self.name.clone());
        self.address = data.address.clone().unwrap_or(self.address.clone());

        Ok(CommunityEvent::Updated(data.clone()))
    }

    pub fn delete(&mut self) -> CommunityEvent {
        self.deleted_at = Some(chrono::Utc::now());
        CommunityEvent::Deleted(CommunityDeletedEvent::new(self.community_id.clone()))
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}
