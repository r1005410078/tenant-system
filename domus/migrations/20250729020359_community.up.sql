-- Add up migration script here
CREATE TABLE favorite_categories (
  id BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '分类主键 ID',
  user_id VARCHAR(50) NOT NULL COMMENT '所属用户 ID',
  name VARCHAR(50) NOT NULL COMMENT '分类名称',
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间'
) COMMENT='用户收藏分类表';

CREATE TABLE user_favorites (
  id BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '收藏记录主键 ID',
  user_id CHAR(36) NOT NULL COMMENT '用户 ID',
  house_id CHAR(36) NOT NULL COMMENT '被收藏的房源 ID',
  category_id BIGINT COMMENT '所属分类 ID（可为空，表示未分类）',
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '收藏时间',

  CONSTRAINT fk_favorite_category
    FOREIGN KEY (category_id) REFERENCES favorite_categories(id)
    ON DELETE SET NULL
) COMMENT='用户收藏房源表';