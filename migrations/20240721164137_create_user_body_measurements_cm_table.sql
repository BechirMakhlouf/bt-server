-- Add migration script here
create table height (
  height FLOAT CHECK (height > 0),
  user_id UUID NOT NULL,
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  PRIMARY KEY (user_id, date_at),
  CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id)
);

create table thighs (
  left_thigh FLOAT CHECK (left_thigh > 0),
  right_thigh FLOAT CHECK (right_thigh > 0),
  user_id UUID NOT NULL,
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  PRIMARY KEY (user_id, date_at),
  CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id)
);

create table neck (
  neck FLOAT CHECK (neck > 0),
  user_id UUID NOT NULL,
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  PRIMARY KEY (user_id, date_at),
  CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id)
);

create table waist (
  waist FLOAT CHECK (waist > 0),
  user_id UUID NOT NULL,
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  PRIMARY KEY (user_id, date_at),
  CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id)
);

create table torso (
  torso FLOAT CHECK (torso > 0),
  user_id UUID NOT NULL,
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  PRIMARY KEY (user_id, date_at),
  CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id)
);

create table hips (
  hips FLOAT CHECK (hips > 0),
  user_id UUID NOT NULL,
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  PRIMARY KEY (user_id, date_at),
  CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id)
);

create table calves (
  left_calf FLOAT CHECK (left_calf > 0),
  right_calf FLOAT CHECK (right_calf > 0),
  user_id UUID NOT NULL,
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  PRIMARY KEY (user_id, date_at),
  CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id)
);

create table arms (
  left_arm FLOAT CHECK (left_arm > 0),
  right_arm FLOAT CHECK (right_arm > 0),
  user_id UUID NOT NULL,
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  PRIMARY KEY (user_id, date_at),
  CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id)
);

create table wrists (
  left_wrist float CHECK (left_wrist > 0),
  right_wrist float CHECK (right_wrist > 0),
  user_id UUID NOT NULL,
  date_at DATE NOT NULL CHECK (date_at <= CURRENT_DATE),
  PRIMARY KEY (user_id, date_at),
  CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id)
);
