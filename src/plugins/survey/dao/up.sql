CREATE TABLE survey_forms(
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT NOT NULL,
  title VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  media_type VARCHAR(8) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);
CREATE INDEX survey_forms_title ON survey_forms(title);

CREATE TABLE survey_fields(
  id BIGSERIAL PRIMARY KEY,
  form_id BIGINT NOT NULL,
  key VARCHAR(32) NOT NULL,
  title VARCHAR(255) NOT NULL,
  required BOOLEAN NOT NULL,
  profile JSON NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);
CREATE UNIQUE INDEX survey_fields_form_key ON survey_fields(form_id, key);

CREATE TABLE survey_responses(
  id BIGSERIAL PRIMARY KEY,
  form_id BIGINT NOT NULL,
  ip VARCHAR(39) NOT NULL,
  body JSON NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);
CREATE UNIQUE INDEX survey_responses_ip ON survey_responses(ip);

CREATE TABLE survey_logs(
  id BIGSERIAL PRIMARY KEY,
  form_id BIGINT NOT NULL,
  user_id BIGINT,
  message TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);
