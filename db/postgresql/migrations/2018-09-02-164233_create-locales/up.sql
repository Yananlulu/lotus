CREATE TABLE locales(
  id BIGSERIAL PRIMARY KEY,
  lang VARCHAR(8) NOT NULL,
  code VARCHAR(255) NOT NULL,
  message TEXT NOT NULL,
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE UNIQUE INDEX idx_locales_lang_code ON locales(lang, code);
CREATE INDEX idx_locales_lang ON locales(lang);
