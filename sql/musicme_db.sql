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

 Date: 16/08/2022 09:45:04
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for favorite
-- ----------------------------
DROP TABLE IF EXISTS `favorite`;
CREATE TABLE `favorite` (
  `user_id` bigint(20) NOT NULL,
  `music_uuid` varchar(36) NOT NULL,
  KEY `userid` (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Table structure for music
-- ----------------------------
DROP TABLE IF EXISTS `music`;
CREATE TABLE `music` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `origin_id` bigint(255) unsigned DEFAULT NULL,
  `name` varchar(255) DEFAULT NULL,
  `ar` varchar(255) DEFAULT NULL,
  `al` varchar(255) DEFAULT NULL,
  `lyric` text DEFAULT NULL,
  `create_date` datetime(6) DEFAULT current_timestamp(6),
  `update_date` datetime(6) DEFAULT NULL ON UPDATE current_timestamp(6),
  PRIMARY KEY (`id`),
  KEY `origin_id` (`origin_id`) USING BTREE,
  KEY `name` (`name`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=12 DEFAULT CHARSET=utf8mb4;

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

-- ----------------------------
-- Table structure for playlist
-- ----------------------------
DROP TABLE IF EXISTS `playlist`;
CREATE TABLE `playlist` (
  `id` bigint(20) DEFAULT NULL,
  `user_id` bigint(20) DEFAULT NULL,
  `name` varchar(255) DEFAULT NULL,
  `carete_time` datetime DEFAULT current_timestamp(),
  KEY `user_id` (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Table structure for playlist2music
-- ----------------------------
DROP TABLE IF EXISTS `playlist2music`;
CREATE TABLE `playlist2music` (
  `playlist_id` bigint(20) NOT NULL,
  `music_id` bigint(20) NOT NULL,
  KEY `playlist` (`playlist_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Table structure for users
-- ----------------------------
DROP TABLE IF EXISTS `users`;
CREATE TABLE `users` (
  `id` bigint(11) NOT NULL AUTO_INCREMENT,
  `username` varchar(255) DEFAULT NULL,
  `password` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `username` (`username`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4;

SET FOREIGN_KEY_CHECKS = 1;
