-- Add migration script here
-- Create Subscriptions Table
CREATE TABLE users (
  id uuid NOT NULL DEFAULT gen_random_uuid(),
  PRIMARY KEY (id),
  hashed_password VARCHAR(1000) NOT NULL,
  email VARCHAR(320) NOT NULL UNIQUE CHECK(email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z]{2,}$'),
  created_at TIMESTAMP NOT NULL DEFAULT now()
);
