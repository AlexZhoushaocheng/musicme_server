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

 Date: 16/08/2022 09:43:14
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for playlist2music
-- ----------------------------
DROP TABLE IF EXISTS `playlist2music`;
CREATE TABLE `playlist2music` (
  `playlist_id` bigint(20) NOT NULL,
  `music_id` bigint(20) NOT NULL,
  KEY `playlist` (`playlist_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

SET FOREIGN_KEY_CHECKS = 1;
