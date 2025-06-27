mod application;
mod domain;
mod infrastructure;
mod init_system;
mod interfaces;
mod start_http_server;
use clap::Parser;
use event_bus::{AsyncEventBus, EventListener};

use crate::application::commands::init_system::{Cli, InitSystemCommand};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(InitSystemCommand::InitSystem {
            admin_name,
            admin_password,
        }) => {
            // 执行初始化逻辑
            init_system::execute(admin_name, admin_password)
                .await
                .unwrap();
            Ok(())
        }
        None => {
            // 启动 HTTP 服务
            println!("启动 HTTP 服务...");
            start_http_server::run().await
        }
    }
}
