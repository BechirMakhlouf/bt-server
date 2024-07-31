use super::user;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {}

#[derive(Default, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "unit", rename_all = "lowercase")]
pub enum Unit {
    #[default]
    Metric,
    Imperial,
}

#[derive(Default, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "theme_mode", rename_all = "lowercase")]
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

impl UsersSettings {
    pub fn new(user_id: user::Id, pref_theme: ThemeMode, pref_unit: Unit) -> Self {
        Self {
            user_id,
            pref_unit,
            pref_theme,
        }
    }
}
