-- Add up migration script here
create table role_aggregate (
  id char(36) primary key not null comment '角色ID',
  name varchar(50) not null comment '角色名称',
  description varchar(100) comment '角色描述',
  deleted_at TIMESTAMP comment '删除时间',
  created_at TIMESTAMP default current_timestamp comment '创建时间',
  updated_at TIMESTAMP default current_timestamp on update current_timestamp comment '更新时间'
);

create table permissions_detail (
  `id` char(36) primary key not null comment '权限ID',
  `name` varchar(50) not null unique comment '权限名称',
  `source` varchar(255) not null comment '权限资源',
  `action` varchar(50) not null comment '权限动作',
  `description` varchar(100) comment '权限描述',
  `created_at` TIMESTAMP default current_timestamp comment '创建时间',
  `updated_at` TIMESTAMP default current_timestamp on update current_timestamp comment '更新时间'
);