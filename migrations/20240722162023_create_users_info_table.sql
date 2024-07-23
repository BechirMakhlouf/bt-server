-- Add migration script here
CREATE TYPE gender as ENUM ('male', 'female', 'other', 'unknown');

CREATE TABLE users_info (
  user_id UUID NOT NULL,
  username VARCHAR(18) UNIQUE NOT NULL CHECK(username ~* '^[A-Za-z0-9_]+$'),
  gender GENDER NOT NULL DEFAULT 'unknown',
  birthday DATE NOT NULL CHECK (birthday <= CURRENT_DATE),
  CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id),
  PRIMARY KEY (user_id)
);
