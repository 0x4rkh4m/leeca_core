use crate::settings::BaseSettings;
use serde::Deserialize;

/// Database configuration.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Database {
    /// The URL of the database.
    pub url: Option<String>,
}

/// Logging configuration.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Logging {
    /// The log level for logging.
    pub log_level: Option<String>,
}

/// Tracing configuration.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Tracing {
    /// The endpoint for OpenTelemetry tracing.
    pub otlp_endpoint: Option<String>,
}

/// General configuration information.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct ConfigInfo {
    /// The location of the configuration file.
    pub location: Option<String>,
    /// The prefix for environment variables.
    pub env_prefix: Option<String>,
}

/// Default settings struct that microservices can use or extend.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Settings {
    #[serde(default)]
    pub config: ConfigInfo,
    #[serde(default)]
    pub database: Database,
    #[serde(default)]
    pub logging: Logging,
}

impl BaseSettings for Settings {}
