-- Add migration script here
create table users_weight_log (
  user_id UUID NOT NULL,
  weight_kg FLOAT NOT NULL,
  weighted_at DATE NOT NULL
);
