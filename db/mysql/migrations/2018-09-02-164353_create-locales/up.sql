CREATE TABLE locales(
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  lang VARCHAR(8) NOT NULL,
  code VARCHAR(255) NOT NULL,
  message TEXT NOT NULL,
  updated_at DATETIME NOT NULL,
  created_at DATETIME NOT NULL
);

CREATE UNIQUE INDEX idx_locales_lang_code ON locales(lang, code);
CREATE INDEX idx_locales_lang ON locales(lang);
