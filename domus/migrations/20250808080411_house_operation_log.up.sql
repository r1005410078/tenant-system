-- Add up migration script here
CREATE TABLE house_operation_log (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    house_id VARCHAR(36) NOT NULL COMMENT '房源ID',
    operation_type TINYINT UNSIGNED NOT NULL COMMENT '操作类型：1=新增，2=修改，3=删除，4=上架，5=下架',
    operation_content JSON NULL COMMENT '操作详情(JSON格式)',
    operator_id VARCHAR(36) NOT NULL COMMENT '操作人ID',
    ip_address VARCHAR(64) NULL COMMENT '操作来源IP',
    user_agent VARCHAR(255) NULL COMMENT '操作来源设备/浏览器',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '操作时间',
    INDEX idx_house_id (house_id),
    INDEX idx_created_at (created_at)
) COMMENT='房源操作记录表';