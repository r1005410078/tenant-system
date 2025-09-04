# ---------- 构建阶段 ----------
# 使用 rust 官方镜像
FROM rust:1.89-slim AS builder

WORKDIR /app
COPY . .

# 安装 musl 工具链并清理缓存
RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*

# 添加 x86_64-musl target
RUN rustup target add x86_64-unknown-linux-musl

# 使用 cargo 构建静态二进制
RUN cargo build --release --target x86_64-unknown-linux-musl --bin domus

# ---------- 运行阶段 ----------
# scratch 是空镜像，最小化镜像体积
FROM scratch AS runner
WORKDIR /app

# 拷贝静态编译好的二进制
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/domus /domus

# 如果有配置文件，也一起拷贝
COPY --from=builder /app/config ./config

# 指定容器启动命令
CMD ["/domus"]