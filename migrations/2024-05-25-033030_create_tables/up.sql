CREATE TABLE `clients` (
  `uuid` char(36) NOT NULL PRIMARY KEY,
  `name` varchar(255) NOT NULL,
  `description` text DEFAULT NULL,
  `archived` tinyint(1) DEFAULT 0,
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

