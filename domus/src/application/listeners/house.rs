use std::sync::Arc;

use event_bus::EventListener;

use crate::{
    application::queries::house::HouseQueryService, domain::house::events::house::HouseEvent,
};

pub struct HouseEventListener {
    pub house_query_service: Arc<HouseQueryService>,
}

impl HouseEventListener {
    pub fn new(house_query_service: Arc<HouseQueryService>) -> Self {
        HouseEventListener {
            house_query_service,
        }
    }
}

#[async_trait::async_trait]
impl EventListener<HouseEvent> for HouseEventListener {
    async fn handle(&self, event: HouseEvent) {
        match event {
            HouseEvent::Created(event) => {
                self.house_query_service.create(event).await.unwrap();
            }
            HouseEvent::Updated(event) => {
                self.house_query_service.update(event).await.unwrap();
            }
            HouseEvent::Published(event) => {
                self.house_query_service
                    .publish(&event.house_id)
                    .await
                    .unwrap();
            }
            HouseEvent::Unpublished(event) => {
                self.house_query_service.unpublish(&event.id).await.unwrap();
            }
            HouseEvent::Deleted(event) => {
                self.house_query_service.delete(&event.id).await.unwrap();
            }
        }
    }
}
