// CREATE TYPE theme_mode as ENUM ('auto', 'dark', 'light');
//
// CREATE TYPE unit as ENUM ('metric', 'imperial');
//
// CREATE TABLE users_settings (
//   user_id UUID NOT NULL,
//   pref_theme THEME_MODE NOT NULL DEFAULT 'auto',
//   pref_unit UNIT NOT NULL DEFAULT 'metric',
//   CONSTRAINT user_id_fk FOREIGN KEY (user_id) REFERENCES users (id),
//   PRIMARY KEY (user_id)
// )
//
use super::user;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {}
#[derive(Default, Debug, Clone)]
pub enum Unit {
    #[default]
    Metric,
    Imperial,
}

#[derive(Default, Debug, Clone)]
pub enum ThemeMode {
    #[default]
    Auto,
    Dark,
    Light,
}

#[derive(Debug, Clone)]
pub struct UsersSettings {
    pub user_id: user::Id,
    pub pref_theme: ThemeMode,
    pub pref_unit: Unit,
}
