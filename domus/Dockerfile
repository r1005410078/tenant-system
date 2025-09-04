# 仅构建阶段
FROM --platform=linux/amd64 rust:1.89 AS builder
WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y musl-tools

RUN rustup target add x86_64-unknown-linux-musl
# 使用 cross 构建 x86_64-musl 静态二进制
RUN cargo build --release --target x86_64-unknown-linux-musl --bin domus

# 第二阶段运行
FROM --platform=linux/amd64 scratch
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/domus /domus
# 拷贝项目中config文件
COPY --from=builder /app/config ./config
CMD ["/domus"]