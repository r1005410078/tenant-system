use clap::Parser;

use crate::application::commands::init_system::{Cli, InitSystemCommand};

mod application;
mod domain;
mod infrastructure;
mod init_system;
mod interfaces;
mod start_http_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(InitSystemCommand::InitSystem) => {
            // 执行初始化逻辑
            init_system::execute().await.unwrap();
            Ok(())
        }
        None => start_http_server::execute().await,
    }
}
