use config::{Config, ConfigError, File, FileFormat};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

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
/// falling back to a default config if that one isn't found.
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
