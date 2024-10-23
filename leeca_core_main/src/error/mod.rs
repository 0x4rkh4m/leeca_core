use thiserror::Error;

/// Core library error enum.
#[derive(Debug, Error)]
pub enum BaseError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Command error: {0}")]
    CommandError(String),

    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Failed to deserialize configuration: {0}")]
    DeserializeError(String),

    #[error("Environment variable not set or invalid: {0}")]
    EnvError(String),

    #[error("Unknown error occurred")]
    Unknown,
}

/// Custom Result type for the core library.
pub type BaseResult<T> = Result<T, BaseError>;

impl BaseError {
    /// Create a new configuration error.
    ///
    /// # Arguments
    /// * `msg` - The error message.
    ///
    /// # Returns
    /// A `BaseError` instance representing a configuration error.
    pub fn config_error(msg: &str) -> Self {
        BaseError::ConfigError(msg.to_string())
    }

    /// Create a new command error.
    ///
    /// # Arguments
    /// * `msg` - The error message.
    ///
    /// # Returns
    /// A `BaseError` instance representing a command error.
    pub fn command_error(msg: &str) -> Self {
        BaseError::CommandError(msg.to_string())
    }
}
