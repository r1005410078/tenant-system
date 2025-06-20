-- Add up migration script here

create table if not exists community_aggregate (
    `id` CHAR(36) PRIMARY KEY NOT NULL COMMENT '小区ID',
    `name` varchar(255) NOT NULL comment '名称',
    `address`  varchar(255) NOT NULL comment '地址',
    `deleted_at` TIMESTAMP comment '删除时间',
    `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);