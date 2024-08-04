use serde::Serialize;

use super::user;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    #[error("Theme mode is invalid: {0}.")]
    InvalidThemeMode(String),
    #[error("Unit is invalid: {0}.")]
    InvalidUnit(String),

    #[error("Invalid user id: {0}")]
    InvalidUserId(String),
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Default, Debug, Clone, Serialize, sqlx::Type)]
#[sqlx(type_name = "unit", rename_all = "lowercase")]
pub enum Unit {
    #[default]
    Metric,
    Imperial,
}

#[derive(Default, Debug, Clone, sqlx::Type, Serialize)]
#[sqlx(type_name = "theme_mode", rename_all = "lowercase")]
pub enum ThemeMode {
    #[default]
    Auto,
    Dark,
    Light,
}

#[derive(Debug, Clone, Serialize)]
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

    pub fn try_from_strs(user_id: &str, pref_theme: &str, pref_unit: &str) -> Result<Self> {
        Ok(Self {
            user_id: user::Id::try_from(user_id)
                .map_err(|_| Error::InvalidUserId(user_id.to_string()))?,
            pref_unit: pref_unit.try_into()?,
            pref_theme: pref_theme.try_into()?,
        })
    }
}

impl TryFrom<&str> for ThemeMode {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value.to_lowercase().as_str() {
            "auto" => Ok(ThemeMode::Auto),
            "dark" => Ok(ThemeMode::Dark),
            "light" => Ok(ThemeMode::Light),
            _ => Err(Error::InvalidThemeMode(value.to_string())),
        }
    }
}

impl TryFrom<&str> for Unit {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        match value.to_lowercase().as_str() {
            "metric" => Ok(Unit::Metric),
            "imperial" => Ok(Unit::Imperial),
            _ => Err(Error::InvalidUnit(value.to_string())),
        }
    }
}
