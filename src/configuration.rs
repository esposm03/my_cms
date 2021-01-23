//! Configuration for the service
//!
//! This module provides the [`get_configuration`] function, which returns
//! an instance of the [`Settings`] struct, containing various configuration
//! options for this application.
//!
//! The options are loaded from the environment, falling back to using defaults.
//! More details can be found in the documentation for [`get_configuration`]

use config::{Config, ConfigError, File, FileFormat};

/// All settings of the application, loaded from the environment
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app_port: u16,
}

/// Application settings related to the database connection
///
/// Information can be extracted from this struct by using its two
/// methods, [`self::connection_string`] and [`self::connection_string_without_db`]
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    /// Access the connection URI that is stored in this configuration struct.
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    /// Access the connection URI stored in this struct, without the logical db part.
    ///
    /// This is useful when you want to create a logical database, as you need to connect
    /// to the main one to do so.
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port,
        )
    }
}

/// Read the configuration from file, falling back to defaults.
///
/// This attempts to read a `configuration.yaml` file in the current directory,
/// falling back to this default config on error:
/// ```yaml
/// app_port: 8000
/// database:
///     host: localhost
///     port: 5432
///     username: "postgres"
///     password: "password"
///     database_name: "cms"
/// ```
pub fn get_configuration() -> Result<Settings, ConfigError> {
    let mut settings = Config::default();
    settings.merge(File::from_str(
        r#"
            app_port: 8000
            database:
                host: localhost
                port: 5432
                username: "postgres"
                password: "password"
                database_name: "cms"
        "#,
        FileFormat::Yaml,
    ))?;
    let _ = settings.merge(File::with_name("configuration"));
    settings.try_into()
}
