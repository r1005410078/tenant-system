docker run  --rm \
  --platform linux/amd64 \
  --network host \
  --name user_system-service \
  -e USER_SYSTEM_DATABASE_URL="mysql://root:123456@192.168.2.10:3306/meida" \
  -e CASBIN_DATABASE_URL="mysql://root:123456@192.168.2.10:3306/meida" \
  registry.cn-hangzhou.aliyuncs.com/tongts/user_system-service:v1:v1