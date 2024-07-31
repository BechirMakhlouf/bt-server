-- Add migration script here
CREATE TYPE gender as ENUM ('male', 'female', 'other', 'unknown');

CREATE TABLE users_info (
  user_id UUID PRIMARY KEY,
  username VARCHAR(18) UNIQUE NOT NULL CHECK(username ~* '^[A-Za-z0-9_]+$'),
  gender GENDER NOT NULL DEFAULT 'unknown',
  birthday DATE NOT NULL CHECK (birthday <= CURRENT_DATE),
  created_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES auth.users
);

CREATE INDEX idx_users_info_user_id ON users_info (user_id);


CREATE TRIGGER handle_updated_at
BEFORE UPDATE ON users_info
FOR EACH ROW
EXECUTE FUNCTION moddatetime();
