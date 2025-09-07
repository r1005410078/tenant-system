# 默认参数
TAG ?= v1
IMAGE_REGISTRY ?= registry.cn-hangzhou.aliyuncs.com/tongts
PLATFORM ?= linux/amd64

# Docker 镜像全名
DOMUS_IMAGE := $(IMAGE_REGISTRY)/domus-service:$(TAG)
USER_SYSTEM_IMAGE := $(IMAGE_REGISTRY)/user_system-service:$(TAG)

.PHONY: domus user_system all

# 构建并推送 domus
domus:
	docker buildx build --platform $(PLATFORM) -t $(DOMUS_IMAGE) --load -f domus/Dockerfile .
	docker push $(DOMUS_IMAGE)

# 构建并推送 user_system
user_system:
	docker buildx build --platform $(PLATFORM) -t $(USER_SYSTEM_IMAGE) --load -f user_system/Dockerfile .
	docker push $(USER_SYSTEM_IMAGE)

# 一次构建并推送所有服务
all: domus user_system