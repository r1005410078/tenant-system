-- Add up migration script here

CREATE TABLE `community_query` (
  `id` VARCHAR(255) NOT NULL,
  `name` VARCHAR(255) NOT NULL,
  `address` VARCHAR(255) NOT NULL,
  `city` VARCHAR(100) NOT NULL,
  `year_built` TIMESTAMP NULL DEFAULT NULL COMMENT '小区建成年份',
  `description` TEXT DEFAULT NULL COMMENT '小区描述',
  `images` JSON DEFAULT NULL COMMENT '小区图片',
  `lat` DOUBLE DEFAULT NULL COMMENT '纬度',
  `lng` DOUBLE DEFAULT NULL COMMENT '经度',
  `typecode` VARCHAR(100) NOT NULL COMMENT '小区类型编码',
  `district` VARCHAR(100) DEFAULT NULL COMMENT '区',
  `adcode` VARCHAR(100) DEFAULT NULL COMMENT '区域编码',  
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='小区读模型表';


CREATE TABLE `owner_query` (
  `id` CHAR(36) PRIMARY KEY NOT NULL COMMENT '业主ID',
  `name` VARCHAR(100) NOT NULL COMMENT '业主姓名',
  `phone` VARCHAR(20) NOT NULL COMMENT '手机号',
  `id_card` VARCHAR(32) DEFAULT NULL COMMENT '身份证号',
  `id_card_images` JSON DEFAULT NULL COMMENT '身份证照片列表',
  `description` TEXT DEFAULT NULL COMMENT '业主情况描述',
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='房东（业主）信息表';

CREATE TABLE `house_query` (
  `id` CHAR(36) PRIMARY KEY NOT NULL COMMENT '房屋ID',
  `community_id` CHAR(36) NOT NULL COMMENT '小区ID',
  `owner_id` CHAR(36) DEFAULT NULL COMMENT '业主ID',

  -- 基本信息
  `title` VARCHAR(255) DEFAULT NULL COMMENT '房源标题',
  `purpose` VARCHAR(50) NOT NULL COMMENT '用途',
  `transaction_type` VARCHAR(50) NOT NULL COMMENT '交易类型',
  `house_status` VARCHAR(50) NOT NULL COMMENT '状态',

  -- 楼层范围
  `door_number_from` INT DEFAULT NULL COMMENT '起始楼层',
  `door_number_to` INT DEFAULT NULL COMMENT '结束楼层',

  -- 门牌号结构
  `building_number` INT DEFAULT NULL COMMENT '栋',
  `unit_number` INT DEFAULT NULL COMMENT '单元',
  `door_number` INT DEFAULT NULL COMMENT '门牌号',
  `current_floor` INT DEFAULT NULL COMMENT '当前楼层',

  -- 户型结构
  `room` INT DEFAULT NULL COMMENT '室',
  `hall` INT DEFAULT NULL COMMENT '厅',
  `bathroom` INT DEFAULT NULL COMMENT '卫',
  `kitchen` INT DEFAULT NULL COMMENT '厨',
  `terrace` INT DEFAULT NULL COMMENT '阳台',
  `balcony` INT DEFAULT NULL COMMENT '阁楼',

  -- 面积与装修
  `building_area` FLOAT DEFAULT NULL COMMENT '建筑面积',
  `use_area` FLOAT DEFAULT NULL COMMENT '使用面积',
  `floor_height` FLOAT DEFAULT NULL COMMENT '层高',
  `house_decoration` VARCHAR(100) DEFAULT NULL COMMENT '装修情况',

  -- 销售租赁信息
  `sale_price` DOUBLE DEFAULT NULL COMMENT '售价',
  `rent_price` DOUBLE DEFAULT NULL COMMENT '租价',
  `rent_low_price` DOUBLE DEFAULT NULL COMMENT '租低价',
  `sale_low_price` DOUBLE DEFAULT NULL COMMENT '售价下限',
  `down_payment` DOUBLE DEFAULT NULL COMMENT '首付',

  -- 房屋结构与产权
  `house_type` VARCHAR(50) DEFAULT NULL COMMENT '房屋类型',
  `house_orientation` VARCHAR(50) DEFAULT NULL COMMENT '朝向',
  `building_structure` VARCHAR(100) DEFAULT NULL COMMENT '建筑结构',
  `building_year` INT DEFAULT NULL COMMENT '建筑年代',
  `property_rights` VARCHAR(100) DEFAULT NULL COMMENT '产权性质',
  `property_year_limit` VARCHAR(50) DEFAULT NULL COMMENT '产权年限',
  `certificate_date` VARCHAR(50) DEFAULT NULL COMMENT '产证日期',
  `handover_date` VARCHAR(50) DEFAULT NULL COMMENT '交房日期',

  -- 标签和特征
  `tags` JSON DEFAULT NULL COMMENT '推荐标签',
  `car_height` DOUBLE DEFAULT NULL COMMENT '车位高度',
  `actual_rate` DOUBLE DEFAULT NULL COMMENT '实率',
  `level` VARCHAR(50) DEFAULT NULL COMMENT '级别',
  `progress_depth` DOUBLE DEFAULT NULL COMMENT '进深',
  `door_width` DOUBLE DEFAULT NULL COMMENT '门宽',

  -- 附加属性
  `discount_year_limit` VARCHAR(50) DEFAULT NULL COMMENT '满减年限',
  `stairs` VARCHAR(20) DEFAULT NULL COMMENT '梯',
  `rooms` VARCHAR(20) DEFAULT NULL COMMENT '户',
  `view_method` VARCHAR(100) DEFAULT NULL COMMENT '看房方式',
  `payment_method` VARCHAR(100) DEFAULT NULL COMMENT '付款方式',
  `property_tax` VARCHAR(100) DEFAULT NULL COMMENT '税费',
  `degree` VARCHAR(100) DEFAULT NULL COMMENT '学位',
  `household` VARCHAR(100) DEFAULT NULL COMMENT '户口',
  `source` VARCHAR(100) DEFAULT NULL COMMENT '来源',
  `delegate_number` VARCHAR(100) DEFAULT NULL COMMENT '委托编号',
  `unique_housing` VARCHAR(10) DEFAULT NULL COMMENT '唯一住房',
  `full_payment` VARCHAR(10) DEFAULT NULL COMMENT '全款',
  `mortgage` VARCHAR(10) DEFAULT NULL COMMENT '抵押',
  `urgent` VARCHAR(10) DEFAULT NULL COMMENT '是否急售',
  `support` TEXT DEFAULT NULL COMMENT '配套',
  `present_state` VARCHAR(100) DEFAULT NULL COMMENT '现状',
  `external_sync` VARCHAR(100) DEFAULT NULL COMMENT '是否同步外网',
  `remark` TEXT DEFAULT NULL COMMENT '备注',
  `images` JSON DEFAULT NULL COMMENT '图片列表',
  `delete_at` TIMESTAMP DEFAULT NULL COMMENT '删除时间',
  -- 创建时间
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='房源表';