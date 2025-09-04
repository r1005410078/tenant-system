docker run -d --rm \
  --platform linux/amd64 \
  --network host \
  --name domus-service \
  -e DOMUS_DATABASE_URL="mysql://root:123456@192.168.2.10:3306/domus" \
  -e CASBIN_DATABASE_URL="mysql://root:123456@192.168.2.10:3306/meida" \
  domus-service:v1