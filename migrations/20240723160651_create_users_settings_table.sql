-- Add migration script here
CREATE TYPE theme_mode as ENUM ('auto', 'dark', 'light');

CREATE TYPE unit as ENUM ('metric', 'imperial');

CREATE TABLE users_settings (
  user_id UUID NOT NULL,
  pref_theme THEME_MODE NOT NULL DEFAULT 'auto',
  pref_unit UNIT NOT NULL DEFAULT 'metric',
  CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id),
  PRIMARY KEY (user_id)
)
