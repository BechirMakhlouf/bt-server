-- Add migration script here
ALTER TABLE USERS ADD COLUMN hashed_password TEXT NOT NULL;
