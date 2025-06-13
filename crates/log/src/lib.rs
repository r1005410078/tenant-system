use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_tracing() {
    let filter = EnvFilter::new("info") // 设置全局日志级别为 debug
        .add_directive("my_app::db=warn".parse().unwrap()) // 单独设置模块日志级别
        .add_directive("tokio=info".parse().unwrap()); // 设定 tokio 相关日志级别

    tracing_subscriber::fmt()
        .with_env_filter(filter) // 支持 RUST_LOG
        // .json() // JSON 格式输出
        .pretty()
        .with_span_events(FmtSpan::CLOSE) // 记录 span 结束信息
        .init();
}
