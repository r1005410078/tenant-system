-- Add up migration script here
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