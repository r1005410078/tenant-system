use std::sync::Arc;

use crate::{
    application::commands::register_user::{RegisterUserCommand, UserRegistrationHandler},
    domain::user::aggregates::user::UserAggregate,
};

pub struct RegisterUserService {
    register_user_handler: Arc<UserRegistrationHandler>,
}

impl RegisterUserService {
    pub fn new(register_user_handler: Arc<UserRegistrationHandler>) -> Self {
        Self {
            register_user_handler,
        }
    }

    pub async fn execute(&self, cmd: RegisterUserCommand) -> anyhow::Result<UserAggregate> {
        self.register_user_handler.handle(cmd).await
    }
}
