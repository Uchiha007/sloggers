use crate::file::FileLoggerConfig;
use crate::null::NullLoggerConfig;
use crate::terminal::TerminalLoggerConfig;
use crate::types::Severity;
use crate::{Build, LoggerBuilder, Result};
use serde::{Deserialize, Serialize};
use slog::Logger;

/// Configuration of a logger builder.
pub trait Config {
    /// Logger builder.
    type Builder: Build;

    /// Makes a logger builder associated with this configuration.
    fn try_to_builder(&self) -> Result<Self::Builder>;

    /// Builds a logger with this configuration.
    fn build_logger(&self) -> Result<Logger> {
        let builder = track!(self.try_to_builder())?;
        let logger = track!(builder.build())?;
        Ok(logger)
    }
}

/// The configuration of `LoggerBuilder`.
///
/// # Examples
///
/// Null logger.
///
/// ```
/// extern crate sloggers;
/// extern crate serdeconv;
///
/// use sloggers::LoggerConfig;
///
/// let toml = r#"
/// type = "null"
/// "#;
/// let _config: LoggerConfig = serdeconv::from_toml_str(toml).unwrap();
/// ```
///
/// Terminal logger.
///
/// ```
/// extern crate sloggers;
/// extern crate serdeconv;
///
/// use sloggers::LoggerConfig;
///
/// let toml = r#"
/// type = "terminal"
/// level = "warning"
/// "#;
/// let _config: LoggerConfig = serdeconv::from_toml_str(toml).unwrap();
/// ```
///
/// File logger.
///
/// ```
/// extern crate sloggers;
/// extern crate serdeconv;
///
/// use sloggers::LoggerConfig;
///
/// let toml = r#"
/// type = "file"
/// path = "/path/to/file.log"
/// timezone = "utc"
/// "#;
/// let _config: LoggerConfig = serdeconv::from_toml_str(toml).unwrap();
/// ```
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum LoggerConfig {
    File(FileLoggerConfig),
    Null(NullLoggerConfig),
    Terminal(TerminalLoggerConfig),
}
impl LoggerConfig {
    /// Sets the log level of this logger.
    pub fn set_loglevel(&mut self, level: Severity) {
        match *self {
            LoggerConfig::File(ref mut c) => c.level = level,
            LoggerConfig::Null(_) => {}
            LoggerConfig::Terminal(ref mut c) => c.level = level,
        }
    }
}
impl Config for LoggerConfig {
    type Builder = LoggerBuilder;
    fn try_to_builder(&self) -> Result<Self::Builder> {
        match *self {
            LoggerConfig::File(ref c) => track!(c.try_to_builder()).map(LoggerBuilder::File),
            LoggerConfig::Null(ref c) => track!(c.try_to_builder()).map(LoggerBuilder::Null),
            LoggerConfig::Terminal(ref c) => {
                track!(c.try_to_builder()).map(LoggerBuilder::Terminal)
            }
        }
    }
}
impl Default for LoggerConfig {
    fn default() -> Self {
        LoggerConfig::Terminal(TerminalLoggerConfig::default())
    }
}
