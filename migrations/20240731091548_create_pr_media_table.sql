-- Add migration script here
CREATE TABLE pr_media (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
  pr_id UUID NOT NULL,
  media_url TEXT NOT NULL,
  created_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pr_id_fk FOREIGN KEY (pr_id) REFERENCES users_pr
);

CREATE TRIGGER handle_updated_at BEFORE
UPDATE ON pr_media FOR EACH ROW EXECUTE FUNCTION moddatetime ();
