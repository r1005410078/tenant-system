use std::sync::Arc;

use crate::{
    application::commands::{
        update_user::{UpdateUserCommand, UpdateUserCommandHandler},
        user_binded_to_roles::{UserBindedToRolesCommand, UserBindedToRolesHandler},
    },
    domain::user::aggregates::user::UserAggregate,
};

pub struct UpdateUserService {
    update_user_command_handler: UpdateUserCommandHandler,
    user_binded_to_roles_command_handler: Arc<UserBindedToRolesHandler>,
}

impl UpdateUserService {
    pub fn new(
        update_user_command_handler: UpdateUserCommandHandler,
        user_binded_to_roles_command_handler: Arc<UserBindedToRolesHandler>,
    ) -> Self {
        Self {
            update_user_command_handler,
            user_binded_to_roles_command_handler,
        }
    }

    pub async fn execute(&self, command: UpdateUserCommand) -> anyhow::Result<UserAggregate> {
        let res = self
            .update_user_command_handler
            .handle(command.clone())
            .await?;

        if let Some(roles) = command.roles {
            self.user_binded_to_roles_command_handler
                .handle(UserBindedToRolesCommand::new(res.id.to_string(), roles))
                .await?;
        }

        Ok(res)
    }
}
