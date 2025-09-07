docker run --rm \
  --platform linux/amd64 \
  --network host \
  --name domus-service \
  -e DOMUS_DATABASE_URL="mysql://root:123456@192.168.2.10:3306/domus" \
  registry.cn-hangzhou.aliyuncs.com/tongts/domus-service:v1