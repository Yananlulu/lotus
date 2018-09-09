CREATE TABLE settings(
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  `key` VARCHAR(255) NOT NULL,
  value BLOB NOT NULL,
  updated_at DATETIME NOT NULL,
  created_at DATETIME NOT NULL
);

CREATE UNIQUE INDEX idx_settings_key ON settings(`key`);
