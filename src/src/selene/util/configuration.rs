use std::{io::Read, path::Path};

use lightlog::{Logger, LoggingLevel, LoggingType};
use serde::{Deserialize, Serialize};

pub const PATH_APPLICATION_CONFIGURATION: &'static str = "configuration/";

#[derive(Serialize, Deserialize, Debug)]
pub struct SeleneConfiguration {
    pub application: ApplicationConfiguration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeleneConnection {
    pub address: String,
    pub port: i16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationConfiguration {
    pub target: SeleneConnection,
    pub proxy: SeleneConnection,
    pub logging: LoggingConfig,
}

impl SeleneConfiguration {
    pub fn get_configuration<T: AsRef<Path>>(file_path: T) -> Option<SeleneConfiguration> {
        let config_file = std::fs::File::open(file_path);

        if config_file.is_err() {
            return None;
        };

        let mut config_file = config_file.unwrap();
        let mut config_data = String::new();

        config_file.read_to_string(&mut config_data).unwrap();

        let configuration = serde_json::from_str(&config_data);

        if configuration.is_err() {
            return None;
        };

        return Some(configuration.unwrap());
    }

    pub fn extract_log_level(&self) -> LoggingLevel {
        match self.application.logging.level.as_str() {
            "FULL" => return LoggingLevel::Full,
            "INFO" => return LoggingLevel::Info,
            "NONE" => return LoggingLevel::None,
            "WARNING" => return LoggingLevel::Warning,
            "ERROR" => return LoggingLevel::Error,
            _ => return LoggingLevel::Full,
        }
    }
}

impl Default for SeleneConfiguration {
    fn default() -> Self {
        let config = SeleneConfiguration::get_configuration(
            PATH_APPLICATION_CONFIGURATION.to_owned() + "config.json",
        );

        if config.is_none() {
            Logger::default().log_origin_message(
                "Configuration couldnt be read.",
                LoggingType::Error,
                Some("Selene"),
            );
            panic!("Configuration couldnt be read.");
        }

        return config.unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingConfig {
    level: String,
}
