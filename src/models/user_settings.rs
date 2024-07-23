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
