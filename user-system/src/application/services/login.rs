use std::sync::Arc;

use crate::application::commands::login::{LoginCommand, LoginCommandHandler};

pub struct LoginService {
    login_handler: Arc<LoginCommandHandler>,
}

impl LoginService {
    pub fn new(login_handler: Arc<LoginCommandHandler>) -> Self {
        Self { login_handler }
    }

    pub async fn execute(&self, cmd: LoginCommand) -> anyhow::Result<String> {
        self.login_handler.handle(cmd).await
    }
}
