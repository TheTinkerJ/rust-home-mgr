create table `market_place`(
		`id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `gmt_create` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `gmt_modified` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `name` varchar(50) NOT NULL COMMENT '消费场所名称',
    `lng` varchar(20) DEFAULT NULL COMMENT '场所 lng 经度',
  	`lat` varchar(20) DEFAULT NULL COMMENT '场所 lat 维度',
    `location_desc` varchar(100) DEFAULT NULL COMMENT '位置描述',
		`adcode` varchar(10) DEFAULT NULL COMMENT '行政编码',
    -- `oss_pics` json DEFAULT NULL COMMENT 'JSON格式的照片存储OSS下载链接', --暂时移除
 	 	PRIMARY KEY (`id`)
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='购买记录';

create table shopping_records(
		`id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `gmt_create` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `gmt_modified` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `place_id` bigint(20) unsigned NOT NULL COMMENT '关联的消费位置',
    `comment` varchar(100) DEFAULT NULL COMMENT '消费整体评价',
    PRIMARY KEY (`id`)
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='购买记录';

create table cost_detail(
		`id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `gmt_create` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `gmt_modified` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `shopping_id` bigint(20) unsigned NOT NULL COMMENT '关联消费记录',
    `good_id` bigint(20) unsigned NOT NULL COMMENT '关联物品',
    `cost` bigint(20) unsigned NOT NULL COMMENT '消费金额(分)',
    `count` bigint(20) unsigned NOT NULL COMMENT '数量',
    `comment` varchar(100) DEFAULT NULL COMMENT '针对消费项的评价',
    PRIMARY KEY (`id`)
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='购买记录';

create table goods(
		`id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `gmt_create` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `gmt_modified` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `name` varchar(50) NOT NULL COMMENT '消费项名称',
    `cls1` varchar(20) DEFAULT NULL COMMENT '一级类目',
    `cls2` varchar(20) DEFAULT NULL COMMENT '二级类目',
    `cls3` varchar(20) DEFAULT NULL COMMENT '三级类目',
    PRIMARY KEY (`id`)
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='消费项';