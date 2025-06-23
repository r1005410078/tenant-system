-- Add up migration script here

CREATE TABLE IF NOT EXISTS role_detail_read_model (
  id char(36) primary key not null comment '角色ID',
  name VARCHAR(255) NOT NULL UNIQUE COMMENT '角色名称',
  description VARCHAR(255) COMMENT '角色描述',
  permissions JSON COMMENT '角色权限',
  created_at TIMESTAMP default current_timestamp comment '创建时间',
  updated_at TIMESTAMP default current_timestamp on update current_timestamp comment '更新时间'
);

 

