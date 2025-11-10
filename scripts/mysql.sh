
docker run -d \
  --name meida \
  -e MYSQL_ROOT_PASSWORD=123456 \
  -v /Users/rongts/tenant-system/data:/var/lib/mysql \
  -p 3306:3306 \
  mysql:8.4

sqlx migrate run --source domus/migrations --database-url mysql://root:123456@localhost/domus
sqlx migrate run --source migrations --database-url mysql://root:123456@localhost/meida