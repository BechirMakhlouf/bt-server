-- Add migration script here
CREATE TYPE theme_mode as ENUM ('auto', 'dark', 'light');

CREATE TYPE unit as ENUM ('metric', 'imperial');

CREATE TABLE users_settings (
  user_id UUID PRIMARY KEY,
  pref_theme THEME_MODE NOT NULL DEFAULT 'auto',
  pref_unit UNIT NOT NULL DEFAULT 'metric',
  created_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES auth.users
);

CREATE INDEX idx_users_settings_user_id ON users_settings (user_id);

CREATE TRIGGER handle_updated_at BEFORE
UPDATE ON users_settings FOR EACH ROW EXECUTE FUNCTION moddatetime ();
