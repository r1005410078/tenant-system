use std::sync::Arc;

use event_bus::AsyncEventBus;
use serde::Deserialize;

use crate::{
    application::{listeners::user_event, repositories::user::UserAggregateRepository},
    domain::user::aggregates::user::UserAggregate,
};

#[derive(Debug, Deserialize)]
pub struct UpdateUserCommand {
    id: String,
    username: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    account_status: Option<String>,
    role: Option<String>,
}

pub struct UpdateUserCommandHandler {
    user_pool: Arc<dyn UserAggregateRepository>,
    event_bus: Arc<AsyncEventBus>,
}

impl UpdateUserCommandHandler {
    pub fn new(user_pool: Arc<dyn UserAggregateRepository>, event_bus: Arc<AsyncEventBus>) -> Self {
        Self {
            user_pool,
            event_bus,
        }
    }

    pub async fn handle(&self, command: UpdateUserCommand) -> anyhow::Result<UserAggregate> {
        let mut user_aggregate = self
            .user_pool
            .find_by_id(&command.id)
            .await
            .ok_or(anyhow::anyhow!("用户不存在"))?;

        let user_event =
            user_aggregate.update(command.username, command.email, command.phone, None);

        self.user_pool.save(&user_aggregate).await?;
        self.event_bus.publish(user_event).await;

        Ok(user_aggregate)
    }
}
