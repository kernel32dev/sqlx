use std::error::Error;

/// Configuration for specific database drivers (**applies to macros and `sqlx-cli` only**).
///
/// # Note: Does Not Apply at Application Run-Time
/// As of writing, these configuration parameters do *not* have any bearing on
/// the runtime configuration of SQLx database drivers.
///
/// See the documentation of individual fields for details.
#[derive(Debug, Default)]
#[cfg_attr(
    feature = "sqlx-toml",
    derive(serde::Deserialize),
    serde(default, rename_all = "kebab-case", deny_unknown_fields)
)]
pub struct Config {
    /// Configuration for the Postgres database driver.
    ///
    /// See [`PgConfig`] for details.
    pub postgres: PgConfig,

    /// Configuration for external database drivers.
    ///
    /// See [`ExternalDriverConfig`] for details.
    pub external: ExternalDriverConfig,
}

/// Configuration for the Postgres database driver.
#[derive(Debug, Default)]
#[cfg_attr(
    feature = "sqlx-toml",
    derive(serde::Deserialize),
    serde(default, rename_all = "kebab-case", deny_unknown_fields)
)]
pub struct PgConfig {
    // No fields implemented yet. This key is only used to validate parsing.
}

/// Configuration for external database drivers.
#[derive(Debug, Default)]
#[cfg_attr(feature = "sqlx-toml", derive(serde::Deserialize), serde(transparent))]
pub struct ExternalDriverConfig {
    #[cfg(feature = "sqlx-toml")]
    by_name: std::collections::BTreeMap<String, toml::Table>,
}

/// Type-erased [`toml::de::Error`].
pub type TryParseError = Box<dyn Error + Send + Sync + 'static>;

impl ExternalDriverConfig {
    /// Try to parse the config for a given driver name, returning `Ok(None)` if it does not exist.
    #[cfg(feature = "sqlx-toml")]
    pub fn try_parse<T: serde::de::DeserializeOwned>(
        &self,
        name: &str,
    ) -> Result<Option<T>, TryParseError> {
        let Some(config) = self.by_name.get(name) else {
            return Ok(None);
        };

        // What's really baffling is that `toml` doesn't provide any way to deserialize
        // from a `&Table` or `&Value`, only owned variants, so cloning is unavoidable here.
        Ok(Some(config.clone().try_into()?))
    }

    /// Try to parse the config for a given driver name, returning `Ok(None)` if it does not exist.
    #[cfg(not(feature = "sqlx-toml"))]
    pub fn try_parse<T>(&self, _name: &str) -> Result<Option<T>, TryParseError> {
        Ok(None)
    }
}
