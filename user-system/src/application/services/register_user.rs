use std::sync::Arc;

use crate::{
    application::commands::{
        register_user::{RegisterUserCommand, UserRegistrationHandler},
        user_binded_to_roles::{UserBindedToRolesCommand, UserBindedToRolesHandler},
    },
    domain::user::aggregates::user::UserAggregate,
};

pub struct RegisterUserService {
    register_user_handler: UserRegistrationHandler,
    user_binded_to_roles_handler: Arc<UserBindedToRolesHandler>,
}

impl RegisterUserService {
    pub fn new(
        register_user_handler: UserRegistrationHandler,
        user_binded_to_roles_handler: Arc<UserBindedToRolesHandler>,
    ) -> Self {
        Self {
            register_user_handler,
            user_binded_to_roles_handler,
        }
    }

    pub async fn execute(&self, cmd: RegisterUserCommand) -> anyhow::Result<UserAggregate> {
        let res = self.register_user_handler.handle(cmd.clone()).await?;

        if let Some(roles) = cmd.roles {
            self.user_binded_to_roles_handler
                .handle(UserBindedToRolesCommand::new(
                    res.id.to_string().clone(),
                    roles,
                ))
                .await?;
        }

        Ok(res)
    }
}
