-- Add migration script here
-- Create Subscriptions Table
CREATE TABLE users (
   id uuid NOT NULL,
   PRIMARY KEY (id),
   email TEXT NOT NULL UNIQUE
);
