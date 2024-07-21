-- Add migration script here
create table user_body_measurements_cm(
  id            uuid    NOT NULL   DEFAULT gen_random_uuid(),
  user_id       uuid    NOT NULL,
  height        float              CHECK(height between 50 and 300), 
  left_arm      float              CHECK(arms between 50 and 200),
  right_arm     float              CHECK(arms between 50 and 200),
  thighs        float              CHECK(thighs between 20 and 300),
  left_wrist    float              CHECK(wrist between 5 and 100),
  right_wrist   float              CHECK(wrist between 5 and 100),
  neck          float              CHECK(neck between 10 and 50),
  left_calf     float              CHECK(calves between 20 and 100),
  right_calf    float              CHECK(calves between 20 and 100),
  hips          float              CHECK(hips between 50 and 400),
  torso         float              CHECK(torso between 50 and 400),
  waist         float              CHECK(waist between 50 and 400),
  date_at       date    NOT NULL   CHECK(date_at <= CURRENT_DATE),

  CONSTRAINT pkey
    PRIMARY KEY (id),
  CONSTRAINT user_id_fk
    FOREIGN KEY (user_id) REFERENCES users(id)
);
