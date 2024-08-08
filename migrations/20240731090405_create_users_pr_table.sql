-- Add migration script here
CREATE TABLE users_pr (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
  user_id UUID NOT NULL,
  title VARCHAR(255) NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  medias TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
  tags VARCHAR(128)[] NOT NULL DEFAULT array[]::varchar[],
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  created_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES auth.users
);

CREATE UNIQUE INDEX idx_users_pr_date_user_id ON users_pr (user_id, date_at);

CREATE TRIGGER handle_updated_at BEFORE
UPDATE ON users_pr FOR EACH ROW EXECUTE FUNCTION moddatetime ();
