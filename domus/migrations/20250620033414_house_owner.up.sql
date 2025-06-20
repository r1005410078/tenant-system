-- Add up migration script here

create table owner (
   `owner_id` CHAR(36) PRIMARY KEY NOT NULL COMMENT '业主ID',
   `name` varchar(255) NOT NULL comment '业主姓名',
   `id_card` varchar(20) comment '业主身份证号',
   `deleted_at` TIMESTAMP comment '删除时间',
   `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
   `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);