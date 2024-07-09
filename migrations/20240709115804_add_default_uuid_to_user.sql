-- Add migration script here
CREATE EXTENSION "uuid-ossp";
ALTER TABLE USERS ALTER COLUMN id SET DEFAULT gen_random_uuid();
