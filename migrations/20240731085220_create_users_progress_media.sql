-- Add migration script here

CREATE TABLE users_progress_media (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid (),
  user_id UUID NOT NULL,
  medias TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  created_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES auth.users
);


CREATE UNIQUE INDEX idx_users_progress_media_date_user_id ON users_progress_media (user_id, date_at);

CREATE TRIGGER handle_updated_at BEFORE
UPDATE ON users_progress_media FOR EACH ROW EXECUTE FUNCTION moddatetime ();
