-- Add up migration script here
create table if not exists user_query (
  user_id char(36) primary key not null comment '用户ID',
  username varchar(50) not null comment '用户名',
  email varchar(100) comment '邮箱',
  phone varchar(20) comment '手机号',
  rules json comment '用户权限',
  created_at TIMESTAMP default current_timestamp comment '创建时间',
  updated_at TIMESTAMP default current_timestamp on update current_timestamp comment '更新时间'
);