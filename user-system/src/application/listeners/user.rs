use event_bus::EventListener;
use sea_orm::*;
use std::sync::Arc;

use crate::{domain::user::events::user_registered::UserRegisteredEvent, infrastructure};

pub struct UserEventListener {
    db: Arc<DbConn>,
}

impl UserEventListener {
    pub fn new(db: Arc<DbConn>) -> Self {
        UserEventListener { db }
    }
}

#[async_trait::async_trait]
impl EventListener<UserRegisteredEvent> for UserEventListener {
    async fn handle(&self, event: UserRegisteredEvent) {
        let user_details = infrastructure::entitiy::user_details_read::ActiveModel {
            id: sea_orm::ActiveValue::Set(event.id.to_string()),
            username: sea_orm::ActiveValue::Set(event.username.clone()),
            email: sea_orm::ActiveValue::Set(event.email.clone()),
            phone: sea_orm::ActiveValue::Set(event.phone.clone()),
            account_status: sea_orm::ActiveValue::Set(event.account_status.to_string()),
            ..Default::default()
        };

        if let Err(err) = user_details.insert(self.db.as_ref()).await {
            println!("Failed to insert user details: {}", err);
            // log::error!("Failed to insert user details: {}", err);
        };
    }
}
