use std::sync::Arc;

use event_bus::AsyncEventBus;

use crate::application::repositories::user::UserAggregateRepository;

pub struct UserBindedToRolesCommand {
    pub user_id: String,
    pub roles: Vec<String>,
}

impl UserBindedToRolesCommand {
    pub fn new(user_id: String, roles: Vec<String>) -> Self {
        Self { user_id, roles }
    }
}

pub struct UserBindedToRolesHandler {
    pub user_pool: Arc<dyn UserAggregateRepository>,
    pub event_bus: Arc<AsyncEventBus>,
}

impl UserBindedToRolesHandler {
    pub async fn handle(&self, command: UserBindedToRolesCommand) -> anyhow::Result<()> {
        let mut user_aggregate = self
            .user_pool
            .find_by_id(&command.user_id)
            .await
            .ok_or(anyhow::anyhow!("用户不存在"))?;

        let event = user_aggregate.bind_roles(command.roles.clone());

        self.user_pool.save(&user_aggregate).await?;
        self.event_bus.publish(event).await;
        Ok(())
    }
}
