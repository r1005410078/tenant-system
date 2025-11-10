#!/bin/bash
STACK_NAME=$1

if [ -z "$STACK_NAME" ]; then
  echo "用法: ./down.sh <STACK_NAME>"
  exit 1
fi

echo "下线 stack: $STACK_NAME ..."
docker stack rm $STACK_NAME

echo "等待容器退出 ..."
sleep 5

echo "停止并删除所有容器 ..."
docker rm -f $(docker ps -aq)

echo "✅ 已下线并清理容器"