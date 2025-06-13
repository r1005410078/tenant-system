use event_bus::AsyncEventBus;
use sea_orm::*;
use std::sync::Arc;

use crate::{domain::user::events::user_registered::UserRegisteredEvent, infrastructure};

pub struct UserEventListener {
    event_bus: Arc<AsyncEventBus>,
    db: Arc<DbConn>,
}

impl UserEventListener {
    pub fn new(event_bus: Arc<AsyncEventBus>, db: Arc<DbConn>) -> Self {
        UserEventListener { event_bus, db }
    }

    pub async fn start_listening(&self) {
        let event_bus_clone = self.event_bus.clone();
        let db = self.db.clone();
        event_bus_clone
            .subscribe(Box::new(move |event: UserRegisteredEvent| {
                let db = db.clone();
                Box::pin(async move {
                    let user_details = infrastructure::entitiy::user_details_read::ActiveModel {
                        id: sea_orm::ActiveValue::Set(event.id.to_string()),
                        username: sea_orm::ActiveValue::Set(event.username.clone()),
                        email: sea_orm::ActiveValue::Set(event.email.clone()),
                        phone: sea_orm::ActiveValue::Set(event.phone.clone()),
                        role: sea_orm::ActiveValue::Set(event.role.clone()),
                        account_status: sea_orm::ActiveValue::Set(event.account_status.to_string()),
                        ..Default::default()
                    };

                    if let Err(err) = user_details.insert(db.as_ref()).await {
                        println!("Failed to insert user details: {}", err);
                        // log::error!("Failed to insert user details: {}", err);
                    };
                })
            }))
            .await;
    }
}
