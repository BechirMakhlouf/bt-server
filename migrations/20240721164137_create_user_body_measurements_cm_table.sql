-- Add migration script here
CREATE TABLE users_body_measurements_cm (
  user_id UUID NOT NULL,
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  left_arm REAL CHECK (left_arm > 0),
  right_arm REAL CHECK (right_arm > 0),
  left_thigh REAL CHECK (left_thigh > 0),
  right_thigh REAL CHECK (right_thigh > 0),
  left_wrist REAL CHECK (left_wrist > 0),
  right_wrist REAL CHECK (right_wrist > 0),
  left_calf REAL CHECK (left_calf > 0),
  right_calf REAL CHECK (right_calf > 0),
  height REAL CHECK (height > 0),
  neck REAL CHECK (neck > 0),
  hips REAL CHECK (hips > 0),
  torso REAL CHECK (torso > 0),
  waist REAL CHECK (waist > 0),
  PRIMARY KEY (user_id, date_at),
  CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX idx_body_measurements_date_user_id ON users_body_measurements_cm (user_id, date_at);
