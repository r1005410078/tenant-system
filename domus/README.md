1. 聚合根（Aggregates）
   • HouseAggregate
   • 负责房源的创建、修改、上下架、绑定小区和房主
   • 相关命令：CreateHouse、UpdateHouse、PublishHouse、UnpublishHouse、BindCommunityToHouse、BindOwnerToHouse
   • 相关事件：HouseCreated、HouseUpdated、HousePublished、HouseUnpublished、CommunityBoundToHouse、OwnerBoundToHouse
   • CommunityAggregate
   • 负责小区信息管理
   • 相关命令：CreateCommunity、UpdateCommunity
   • 相关事件：CommunityCreated、CommunityUpdated
   • OwnerAggregate
   • 负责房主信息管理
   • 相关命令：CreateOwner、UpdateOwner
   • 相关事件：OwnerCreated、OwnerUpdated

![assets/alt text](image.png)
![assets/alt text](image-1.png)

4. 领域模型类（Domain Models）
   • House（实体）
   • Community（实体）
   • Owner（实体）
   • HouseStatus（值对象/枚举，表示房源状态）
   • CommunityInfo（值对象，包含小区详细信息）
   • OwnerInfo（值对象，包含房主详细信息）

HouseAggregate
├── 维护 House 实体
├── 处理 House 相关命令
└── 发布 House 相关事件
├── 触发 Community 相关命令（如自动创建小区）
└── 触发 Owner 相关命令（如绑定房主）

CommunityAggregate
├── 维护 Community 实体
├── 处理 Community 相关命令
└── 发布 Community 相关事件

OwnerAggregate
├── 维护 Owner 实体
├── 处理 Owner 相关命令
└── 发布 Owner 相关事件
