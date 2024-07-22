-- Add migration script here
create table users_weight_log (
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  user_id UUID NOT NULL,
  weight_kg FLOAT NOT NULL CHECK (weight_kg between 5 and 1000),
  PRIMARY KEY (user_id, date_at),
  CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX idx_users_weight_log_date_user_id ON users_weight_log (user_id, date_at)
