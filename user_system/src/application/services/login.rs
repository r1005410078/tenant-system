use crate::application::commands::login::{LoginCommand, LoginCommandHandler};

pub struct LoginService {
    login_handler: LoginCommandHandler,
}

impl LoginService {
    pub fn new(login_handler: LoginCommandHandler) -> Self {
        Self { login_handler }
    }

    pub async fn execute(&self, cmd: LoginCommand) -> anyhow::Result<String> {
        self.login_handler.handle(cmd).await
    }
}
