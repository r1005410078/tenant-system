use crate::application::commands::login::{LoginCommand, LoginCommandHandler, LoginInfomation};

pub struct LoginService {
    login_handler: LoginCommandHandler,
}

impl LoginService {
    pub fn new(login_handler: LoginCommandHandler) -> Self {
        Self { login_handler }
    }

    pub async fn execute(&self, cmd: LoginCommand) -> anyhow::Result<LoginInfomation> {
        self.login_handler.handle(cmd).await
    }
}
