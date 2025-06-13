use crate::domain::user::events::login::LoginEvent;
use crate::infrastructure::entitiy::sea_orm_active_enums::Status;
use crate::infrastructure::entitiy::user_login_history;
use event_bus::AsyncEventBus;
use sea_orm::*;
use std::sync::Arc;
pub struct LoginEventListener {
    event_bus: Arc<AsyncEventBus>,
    db: Arc<DbConn>,
}

impl LoginEventListener {
    pub fn new(event_bus: Arc<AsyncEventBus>, db: Arc<DbConn>) -> Self {
        LoginEventListener { event_bus, db }
    }

    pub async fn start_listening(&self) {
        let event_bus_clone = self.event_bus.clone();
        let db = self.db.clone();
        event_bus_clone
            .subscribe(Box::new(move |event: LoginEvent| {
                let db = db.clone();
                Box::pin(async move {
                    let user_details = match event {
                        LoginEvent::Success(event) => user_login_history::ActiveModel {
                            username: sea_orm::ActiveValue::Set(event.username.clone()),
                            login_at: sea_orm::ActiveValue::Set(event.login_time.naive_utc()),
                            status: sea_orm::ActiveValue::Set(Status::Success),
                            ..Default::default()
                        },
                        LoginEvent::Fail(event) => user_login_history::ActiveModel {
                            username: sea_orm::ActiveValue::Set(event.username.clone()),
                            login_at: sea_orm::ActiveValue::Set(event.login_time.naive_utc()),
                            status: sea_orm::ActiveValue::Set(Status::Failure),
                            ..Default::default()
                        },
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
