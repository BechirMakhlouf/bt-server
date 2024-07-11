-- Add migration script here
CREATE TABLE sessions (
  id uuid NOT NULL,
  user_id uuid NOT NULL,
  created_at timestamp not null default now(),
  FOREIGN KEY (user_id)
    REFERENCES users (id),
  PRIMARY KEY (id)
);
