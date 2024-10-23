mod config_structs;
pub use config_structs::Settings;

use crate::error::{BaseError, BaseResult};
use config::{Config, Environment, File};
use serde::de::DeserializeOwned;

/// A trait that defines the basic configuration behavior for settings.
pub trait BaseSettings: DeserializeOwned + Default + Clone {
    /// Load settings from a configuration file and environment variables.
    ///
    /// # Arguments
    /// * `location` - The location of the configuration file.
    /// * `env_prefix` - The prefix for environment variables.
    ///
    /// # Returns
    /// A `BaseResult` containing the loaded settings or an error.
    fn load(location: &str, env_prefix: &str) -> BaseResult<Self> {
        let builder = Config::builder()
            .add_source(File::with_name(location).required(true))
            .add_source(
                Environment::with_prefix(env_prefix)
                    .separator("__")
                    .prefix_separator("__"),
            )
            .set_override("config.location", location)
            .map_err(|e| BaseError::config_error(&format!("Failed to set config location: {}", e)))?
            .set_override("config.env_prefix", env_prefix)
            .map_err(|e| BaseError::config_error(&format!("Failed to set env prefix: {}", e)))?;

        let config = builder
            .build()
            .map_err(|e| BaseError::config_error(&format!("Failed to build config: {}", e)))?;

        let settings: Self = config
            .try_deserialize()
            .map_err(|e| BaseError::DeserializeError(e.to_string()))?;

        Ok(settings)
    }
}
