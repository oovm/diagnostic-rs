use crate::DokiError;
use config::ConfigError;

impl From<ConfigError> for DokiError {
    fn from(e: ConfigError) -> Self {
        Self::runtime_error(e.to_string())
    }
}
