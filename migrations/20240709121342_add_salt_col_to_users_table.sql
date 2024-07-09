-- Add migration script here
ALTER TABLE USERS ADD COLUMN salt text not null;
