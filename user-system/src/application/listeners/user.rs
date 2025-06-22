use event_bus::EventListener;
use sea_orm::*;
use std::sync::Arc;

use crate::{
    application::queries::user_query_service::UserQueryService,
    domain::user::events::{user::UserEvent, user_registered::UserRegisteredEvent},
    infrastructure,
};

pub struct UserEventListener {
    user_query_service: Arc<UserQueryService>,
}

impl UserEventListener {
    pub fn new(user_query_service: Arc<UserQueryService>) -> Self {
        UserEventListener { user_query_service }
    }
}

#[async_trait::async_trait]
impl EventListener<UserEvent> for UserEventListener {
    async fn handle(&self, event: UserEvent) {
        match event {
            UserEvent::UserRegistered(event) => {
                self.user_query_service.save_user(&event).await.unwrap();
            }

            UserEvent::UserUpdated(event) => {
                self.user_query_service.update_user(&event).await.unwrap();
            }

            UserEvent::UserDeleted(event) => {
                self.user_query_service
                    .delete_user(&event.id)
                    .await
                    .unwrap();
            }

            UserEvent::UserBindedToRoles(event) => {
                self.user_query_service.bind_roles(&event).await.unwrap();
            }
        }
    }
}
