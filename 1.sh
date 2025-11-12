docker run -d \
  -p 9000:9000 -p 9001:9001 \
  -e "MINIO_ROOT_USER=admin" \
  -e "MINIO_ROOT_PASSWORD=rts2778205" \
  -v /mnt/data/minio:/data \
  --name minio \
  quay.io/minio/minio server /data --console-address ":9001"