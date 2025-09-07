-- Add up migration script here
CREATE TABLE user_aggregate (
  id CHAR(36) PRIMARY KEY NOT NULL COMMENT '用户ID',
  username VARCHAR(50) NOT NULL COMMENT '用户名',
  email VARCHAR(100) COMMENT '邮箱',
  phone VARCHAR(20) COMMENT '手机号',
  password VARCHAR(100) NOT NULL COMMENT '密码',
  account_status VARCHAR(50) NOT NULL COMMENT '账户状态',
  register_time TIMESTAMP NOT NULL COMMENT '注册时间',
  last_login_time TIMESTAMP NULL COMMENT '最后登录时间',
  deleted_at TIMESTAMP NULL COMMENT '删除时间',
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

CREATE TABLE `event_record` (
  `event_id` CHAR(36) NOT NULL COMMENT '事件唯一标识 UUID',
  `event_type` VARCHAR(100) NOT NULL COMMENT '事件类型（如 HouseCreated）',
  `payload` JSON NOT NULL COMMENT '事件负载（原始事件数据）',

  `status` VARCHAR(20) NOT NULL DEFAULT 'pending' COMMENT '事件状态（pending, published, failed）',
  `retry_count` INT NOT NULL DEFAULT 0 COMMENT '重试次数',

  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '事件创建时间',
  `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',

  PRIMARY KEY (`event_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='事件记录表（用于事件总线和日志）';