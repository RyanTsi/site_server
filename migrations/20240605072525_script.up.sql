CREATE TABLE `users` (
    `uid` INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `passwd` VARCHAR(255) NOT NULL,
    `avatar` VARCHAR(255),
    `name` VARCHAR(255),
    `bio` TEXT DEFAULT NULL,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) AUTO_INCREMENT=10000000;

CREATE TABLE `posts` (
    `pid` uuid NOT NULL DEFAULT uuid() PRIMARY KEY,
    `title` varchar(255) NOT NULL,
    `author` INT UNSIGNED,
    `brief` text NOT NULL DEFAULT '暂无简介',
    `content` longtext DEFAULT NULL,
    `created_at` timestamp NULL DEFAULT current_timestamp(),
    `updated_at` timestamp NULL DEFAULT current_timestamp() ON UPDATE current_timestamp()
);

CREATE TABLE `remarks` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `uid` INT UNSIGNED DEFAULT NULL,
  `pid` uuid DEFAULT NULL,
  `content` text DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT current_timestamp(),
  `updated_at` timestamp NULL DEFAULT current_timestamp() ON UPDATE current_timestamp()
) AUTO_INCREMENT=0;

CREATE TABLE `chat_messages` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `uid` INT UNSIGNED DEFAULT NULL,
  `content` text DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT current_timestamp(),
  `updated_at` timestamp NULL DEFAULT current_timestamp() ON UPDATE current_timestamp()
) AUTO_INCREMENT=0;

CREATE TABLE `post_tag` (
  `pid` uuid NOT NULL,
  `tag` varchar(255) NOT NULL,
  PRIMARY KEY (`tag`,`pid`)
);