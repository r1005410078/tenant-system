use crate::{
    application::commands::update_user::{UpdateUserCommand, UpdateUserCommandHandler},
    domain::user::aggregates::user::UserAggregate,
};

pub struct UpdateUserService {
    update_user_command_handler: UpdateUserCommandHandler,
}

impl UpdateUserService {
    pub fn new(update_user_command_handler: UpdateUserCommandHandler) -> Self {
        Self {
            update_user_command_handler,
        }
    }

    pub async fn execute(&self, command: UpdateUserCommand) -> anyhow::Result<UserAggregate> {
        self.update_user_command_handler.handle(command).await
    }
}
