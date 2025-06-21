use crate::{
    application::queries::user_query_service::UserQueryService,
    domain::user::events::login::LoginEvent,
};
use event_bus::EventListener;
use std::sync::Arc;

pub struct LoginEventListener {
    user_query_service: Arc<UserQueryService>,
}

impl LoginEventListener {
    pub fn new(user_query_service: Arc<UserQueryService>) -> Self {
        LoginEventListener { user_query_service }
    }
}

#[async_trait::async_trait]
impl EventListener<LoginEvent> for LoginEventListener {
    async fn handle(&self, event: LoginEvent) {
        self.user_query_service
            .save_user_login_history(&event)
            .await
            .unwrap();
    }
}
