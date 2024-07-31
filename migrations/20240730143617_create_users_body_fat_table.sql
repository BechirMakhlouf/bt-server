-- Add migration script here
create table users_body_fat (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid (),
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  user_id UUID NOT NULL,
  body_fat REAL NOT NULL CHECK (body_fat between 0 and 100),
  created_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES auth.users
);

CREATE INDEX idx_users_body_fat_date_user_id ON users_body_fat (user_id, date_at);

CREATE TRIGGER handle_updated_at BEFORE
UPDATE ON users_body_fat FOR EACH ROW EXECUTE FUNCTION moddatetime ();
