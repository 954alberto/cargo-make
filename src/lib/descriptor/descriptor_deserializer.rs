//! # descriptor_deserializer
//!
//! Deserializes and validates the configs.

#[cfg(test)]
#[path = "descriptor_deserializer_test.rs"]
mod descriptor_deserializer_test;

use crate::error::CargoMakeError;
use crate::types::{Config, ExternalConfig};

pub(crate) fn load_config(
    descriptor_string: &str,
    validate: bool,
) -> Result<Config, CargoMakeError> {
    let config: Config = if validate {
        let deserializer = toml::de::Deserializer::new(descriptor_string);

        match serde_ignored::deserialize(deserializer, |path| {
            error!("Found unknown key: {}", path);
        }) {
            Ok(value) => value,
            Err(error) => {
                error!("Unable to parse internal descriptor: {}", error);
                return Err(CargoMakeError::DescriptorParseFailed(error.to_string()));
            }
        }
    } else {
        match toml::from_str(descriptor_string) {
            Ok(value) => value,
            Err(error) => {
                error!("Unable to parse internal descriptor: {}", error);
                return Err(CargoMakeError::DescriptorParseFailed(error.to_string()));
            }
        }
    };

    Ok(config)
}

pub(crate) fn load_external_config(
    descriptor_string: &str,
    file: &str,
) -> Result<ExternalConfig, CargoMakeError> {
    let deserializer = toml::de::Deserializer::new(descriptor_string);

    match serde_ignored::deserialize(deserializer, |path| {
        warn!("Found unknown key: {} in file: {}", path, file);
    }) {
        Ok(value) => Ok(value),
        Err(error) => {
            error!("Unable to parse external file: {:#?}, {}", &file, error);
            return Err(CargoMakeError::ParseFileFailed(
                String::from(file),
                error.to_string(),
            ));
        }
    }
}
