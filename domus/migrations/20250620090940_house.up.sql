-- Add up migration script here

create table house_aggregate (
   `house_id` CHAR(36) PRIMARY KEY NOT NULL COMMENT '房屋ID',
   `community_id` CHAR(36) NOT NULL COMMENT '小区ID',
   `house_address` varchar(255) comment '地址（楼号/单元号/门牌号）',
   `publish_at` TIMESTAMP comment '上架时间',
   `unpublish_at` TIMESTAMP comment '下架时间',
   `deleted_at` TIMESTAMP comment '删除时间',
   `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
   `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);