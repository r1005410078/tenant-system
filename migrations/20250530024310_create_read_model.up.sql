-- Add up migration script here

-- 用户详情读模型
CREATE TABLE user_details_read (
  id CHAR(36) PRIMARY KEY NOT NULL COMMENT '用户ID',
  username VARCHAR(50) NOT NULL UNIQUE COMMENT '用户名',
  email VARCHAR(100) COMMENT '邮箱',
  phone VARCHAR(20) COMMENT '手机号',
  role VARCHAR(50) NOT NULL COMMENT '角色',
  account_status VARCHAR(50) NOT NULL COMMENT '账户状态',   
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 用户登录历史查询模型：记录用户的登录 历史，用于审计和查询。

CREATE TABLE `user_login_history` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键',
  `user_id` BIGINT UNSIGNED DEFAULT NULL COMMENT '用户 ID，如果为 NULL 表示登录失败找不到用户',
  `username` VARCHAR(64) NOT NULL COMMENT '尝试登录的用户名',
  `status` ENUM('SUCCESS', 'FAILURE') NOT NULL COMMENT '登录状态',
  `ip_address` VARCHAR(45) DEFAULT NULL COMMENT '登录 IP 地址 (支持 IPv6)',
  `user_agent` VARCHAR(255) DEFAULT NULL COMMENT '客户端信息 (浏览器、系统等)',
  `login_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '登录时间',
  `fail_reason` VARCHAR(255) DEFAULT NULL COMMENT '失败原因（如密码错误、用户不存在）',
  `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',

  PRIMARY KEY (`id`),
  INDEX `idx_user_id_login_at` (`user_id`, `login_at`),
  INDEX `idx_username_login_at` (`username`, `login_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户登录历史记录表';


CREATE TABLE `user_operation_log` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键',
  `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户 ID',
  `username` VARCHAR(64) NOT NULL COMMENT '用户名（冗余）',
  `action` VARCHAR(64) NOT NULL COMMENT '操作动作（如 delete_house, update_profile）',
  `resource_type` VARCHAR(64) DEFAULT NULL COMMENT '资源类型（如 house, profile）',
  `resource_id` VARCHAR(64) DEFAULT NULL COMMENT '资源 ID（如房源 ID）',
  `ip_address` VARCHAR(45) DEFAULT NULL COMMENT '操作 IP',
  `user_agent` VARCHAR(255) DEFAULT NULL COMMENT '客户端信息',
  `operation_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '操作时间',
  `description` TEXT DEFAULT NULL COMMENT '操作说明或上下文 JSON',
  PRIMARY KEY (`id`),
  INDEX `idx_user_id_operation_at` (`user_id`, `operation_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户操作日志表';