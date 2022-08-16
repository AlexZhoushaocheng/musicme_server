/*
 Navicat Premium Data Transfer

 Source Server         : alex
 Source Server Type    : MariaDB
 Source Server Version : 100803
 Source Host           : www.litesite.cn:3306
 Source Schema         : musicme

 Target Server Type    : MariaDB
 Target Server Version : 100803
 File Encoding         : 65001

 Date: 16/08/2022 09:41:34
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for musicme
-- ----------------------------
DROP TABLE IF EXISTS `musicme`;
CREATE TABLE `musicme` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `uuid` varchar(36) DEFAULT NULL,
  `name` varchar(128) DEFAULT NULL,
  `type` varchar(8) DEFAULT NULL,
  `ar` varchar(128) DEFAULT NULL,
  `al` varchar(128) DEFAULT NULL,
  `lyric` text DEFAULT NULL,
  `create_date` datetime(6) DEFAULT current_timestamp(6),
  `update_date` datetime(6) DEFAULT NULL ON UPDATE current_timestamp(6),
  PRIMARY KEY (`id`),
  KEY `uuid` (`uuid`) USING BTREE,
  KEY `name` (`name`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=14 DEFAULT CHARSET=utf8mb4;

SET FOREIGN_KEY_CHECKS = 1;
