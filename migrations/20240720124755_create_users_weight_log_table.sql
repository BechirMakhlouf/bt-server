-- Add migration script here
create table users_weight (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid (),
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  user_id UUID NOT NULL,
  weight_kg REAL NOT NULL CHECK (weight_kg between 5 and 1000),
  created_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES auth.users
);

CREATE UNIQUE INDEX idx_users_weight_date_user_id ON users_weight (user_id, date_at);

CREATE TRIGGER handle_updated_at BEFORE
UPDATE ON users_weight FOR EACH ROW EXECUTE FUNCTION moddatetime ();
