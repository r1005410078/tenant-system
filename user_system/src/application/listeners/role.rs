use std::sync::Arc;

use event_bus::EventListener;

use crate::{
    application::queries::role_query_service::RoleQueryService,
    domain::roles::events::role::RoleEvent,
};

pub struct RoleEventListener {
    role_query_service: Arc<RoleQueryService>,
}

impl RoleEventListener {
    pub fn new(role_query_service: Arc<RoleQueryService>) -> Self {
        Self { role_query_service }
    }
}

#[async_trait::async_trait]
impl EventListener<RoleEvent> for RoleEventListener {
    async fn handle(&self, event: RoleEvent) {
        match event {
            RoleEvent::RoleCreated(event) => {
                self.role_query_service.create(event).await.unwrap();
            }
            RoleEvent::RoleUpdated(event) => {
                self.role_query_service.update(event).await.unwrap();
            }
            RoleEvent::RoleDeleted(event) => {
                self.role_query_service.delete(event).await.unwrap();
            }
            RoleEvent::PermissionGrantedToRole(event) => {
                self.role_query_service
                    .bind_permissions(&event)
                    .await
                    .unwrap();
            }
        }
    }
}
