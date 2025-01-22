/*
SQLyog 企业版 - MySQL GUI v8.14 
MySQL - 5.5.5-10.11.2-MariaDB : Database - investmentresearch
*********************************************************************
*/


/*!40101 SET NAMES utf8 */;

/*!40101 SET SQL_MODE=''*/;

/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;
CREATE DATABASE /*!32312 IF NOT EXISTS*/`InvestmentResearch` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci */;

USE `InvestmentResearch`;

/*Table structure for table `daily_data` */

DROP TABLE IF EXISTS `daily_data`;

CREATE TABLE `daily_data` (
  `id` int(7) NOT NULL DEFAULT 0
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

/*Table structure for table `equity_fetch_history` */

DROP TABLE IF EXISTS `equity_fetch_history`;

CREATE TABLE `equity_fetch_history` (
  `id` int(5) NOT NULL AUTO_INCREMENT,
  `fetch_date` varchar(12) NOT NULL,
  `equity_type` varchar(10) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

/*Table structure for table `fund` */

DROP TABLE IF EXISTS `fund`;

CREATE TABLE `fund` (
  `id` int(7) NOT NULL AUTO_INCREMENT,
  `ts_code` varchar(10) NOT NULL,
  `name` varchar(20) NOT NULL,
  `simple_name` varchar(15) NOT NULL,
  `management` varchar(10) DEFAULT NULL,
  `custodian` varchar(10) DEFAULT NULL,
  `fund_type` varchar(10) DEFAULT NULL,
  `found_date` varchar(10) DEFAULT NULL,
  `due_date` varchar(10) DEFAULT NULL,
  `list_date` varchar(10) DEFAULT NULL,
  `issue_date` varchar(10) DEFAULT NULL,
  `delist_date` varchar(10) DEFAULT NULL,
  `issue_amount` double DEFAULT NULL,
  `m_fee` double DEFAULT NULL,
  `c_fee` double DEFAULT NULL,
  `duration_year` int(3) DEFAULT NULL,
  `p_value` double DEFAULT NULL,
  `min_amount` double DEFAULT NULL,
  `exp_return` double DEFAULT NULL,
  `benchmark` varchar(100) DEFAULT NULL,
  `status` varchar(10) DEFAULT NULL,
  `invest_type` varchar(10) DEFAULT NULL,
  `type_` varchar(10) DEFAULT NULL,
  `trustee` double DEFAULT NULL,
  `purc_startdate` varchar(10) DEFAULT NULL,
  `redm_startdate` varchar(10) DEFAULT NULL,
  `market` varchar(10) DEFAULT NULL,
  PRIMARY KEY (`id`,`ts_code`,`name`,`simple_name`),
  UNIQUE KEY `ts_code` (`ts_code`)
) ENGINE=InnoDB AUTO_INCREMENT=3630 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

/*Table structure for table `monthly_data` */

DROP TABLE IF EXISTS `monthly_data`;

CREATE TABLE `monthly_data` (
  `id` int(7) NOT NULL AUTO_INCREMENT,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

/*Table structure for table `stock` */

DROP TABLE IF EXISTS `stock`;

CREATE TABLE `stock` (
  `id` int(6) NOT NULL AUTO_INCREMENT,
  `name` varchar(30) NOT NULL,
  `ts_code` varchar(10) NOT NULL,
  `country` varchar(10) DEFAULT NULL,
  `symbol` varchar(10) DEFAULT NULL,
  `area` varchar(10) DEFAULT NULL,
  `list_date` varchar(10) DEFAULT NULL,
  `industry` varchar(10) DEFAULT NULL,
  `simple_name` varchar(10) DEFAULT NULL,
  `market` varchar(10) DEFAULT NULL,
  `exchange` varchar(10) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `NewIndex1` (`ts_code`),
  UNIQUE KEY `NewIndex2` (`symbol`),
  KEY `list_date` (`list_date`)
) ENGINE=InnoDB AUTO_INCREMENT=26179 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

/*Table structure for table `stock_daily` */

DROP TABLE IF EXISTS `stock_daily`;

CREATE TABLE `stock_daily` (
  `id` int(8) NOT NULL AUTO_INCREMENT,
  `ts_code` varchar(10) NOT NULL,
  `trade_date` varchar(10) NOT NULL,
  `open` double NOT NULL,
  `high` double NOT NULL,
  `low` double NOT NULL,
  `close` double NOT NULL,
  `pre_close` double NOT NULL,
  `change` double NOT NULL,
  `pct_chg` double NOT NULL,
  `vol` double NOT NULL,
  `amount` double DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `tscode_tradedate` (`ts_code`,`trade_date`),
  KEY `trade_date` (`trade_date`)
) ENGINE=InnoDB AUTO_INCREMENT=72159466 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

/*Table structure for table `stock_daily_fetch_record` */

DROP TABLE IF EXISTS `stock_daily_fetch_record`;

CREATE TABLE `stock_daily_fetch_record` (
  `id` int(7) NOT NULL AUTO_INCREMENT,
  `fetch_date` varchar(10) NOT NULL,
  `day_of_week` int(3) NOT NULL,
  `num_of_stock` int(5) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `NewIndex1` (`fetch_date`)
) ENGINE=InnoDB AUTO_INCREMENT=77117 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
