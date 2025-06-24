use std::sync::Arc;

use event_bus::EventListener;

use crate::{
    application::queries::owner::OwnerQueryService, domain::owner::events::owner::OwnerEvent,
};

pub struct OwnerEventListener {
    owner_query_service: Arc<OwnerQueryService>,
}

impl OwnerEventListener {
    pub fn new(owner_query_service: Arc<OwnerQueryService>) -> Self {
        OwnerEventListener {
            owner_query_service,
        }
    }
}

#[async_trait::async_trait]
impl EventListener<OwnerEvent> for OwnerEventListener {
    async fn handle(&self, event: OwnerEvent) {
        match event {
            OwnerEvent::Created(event) => {
                self.owner_query_service.create(event).await.unwrap();
            }
            OwnerEvent::Updated(event) => {
                self.owner_query_service.update(event).await.unwrap();
            }
            OwnerEvent::Deleted(event) => {
                self.owner_query_service
                    .delete(&event.owner_id)
                    .await
                    .unwrap();
            }
        }
    }
}
