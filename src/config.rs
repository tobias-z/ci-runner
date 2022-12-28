use std::fs;

use serde::Deserialize;

use crate::{constants, types::CIError};

#[derive(Deserialize)]
pub struct Config {
    pub version: f32,
}

pub fn load_config() -> Result<Config, CIError> {
    let ci_file = fs::read_to_string(format!("{}/ci-file.yaml", constants::WORKSPACE_DIR));
    if let Err(_) = ci_file {
        return Err(Box::from(format!(
            "unable to find CI file in the directory: {}",
            constants::WORKSPACE_DIR
        )));
    }
    match map_to_config(ci_file.unwrap()) {
        Ok(val) => Ok(val),
        Err(err) => Err(Box::from(err)),
    }
}

fn map_to_config(config_str: String) -> Result<Config, serde_yaml::Error> {
    serde_yaml::from_str::<Config>(&config_str)
}

#[cfg(test)]
mod test {
    use super::*;

    const VALID_CONFIG: &str = "
        version: 0.1
    ";

    #[test]
    fn config_contains_version() {
        let config = map_to_config(VALID_CONFIG.to_string());
        assert!(config.is_ok());
        assert_eq!(config.unwrap().version, 0.1);
    }

    #[test]
    fn given_incorrect_format_mapping_will_fail() {
        let incorrect_config = "
            something-wrong: 0.1
        ";
        let config = map_to_config(incorrect_config.to_string());
        assert!(config.is_err());
    }

    #[test]
    fn given_incorrect_type_will_fail() {
        let incorrect_config = "
            version: something
        ";
        let config = map_to_config(incorrect_config.to_string());
        assert!(config.is_err());
    }
}
