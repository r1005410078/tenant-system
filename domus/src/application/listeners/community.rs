use std::sync::Arc;

use event_bus::EventListener;

use crate::{
    application::queries::community::CommunityQueryService,
    domain::community::events::community::CommunityEvent,
};

pub struct CommunityEventListener {
    community_query_service: Arc<CommunityQueryService>,
}

impl CommunityEventListener {
    pub fn new(community_query_service: Arc<CommunityQueryService>) -> Self {
        CommunityEventListener {
            community_query_service,
        }
    }
}

#[async_trait::async_trait]
impl EventListener<CommunityEvent> for CommunityEventListener {
    async fn handle(&self, event: CommunityEvent) {
        match event {
            CommunityEvent::Created(event) => {
                self.community_query_service.create(event).await.unwrap();
            }
            CommunityEvent::Updated(event) => {
                self.community_query_service.update(event).await.unwrap();
            }
            CommunityEvent::Deleted(event) => {
                self.community_query_service
                    .delete(&event.id)
                    .await
                    .unwrap();
            }
        }
    }
}
