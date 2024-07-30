-- Add migration script here
CREATE TABLE users_profiles (
  user_id UUID PRIMARY KEY,
  --TODO: change the text type here and maybe add regex for validation
  picture_url TEXT not null,
  url varchar(96) not null,
  description TEXT not null default '',
  created_at TIMESTAMP
  WITH
    TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES auth.users
);

CREATE INDEX idx_users_profiles_user_id ON users_profiles (user_id);

CREATE TRIGGER handle_updated_at BEFORE
UPDATE ON users_profiles FOR EACH ROW EXECUTE FUNCTION moddatetime ();
