mod runner_validator;

use crate::config;

type ConfigValidator = fn(config::Config) -> Option<config::Config>;

static CONFIG_VALIDATORS: &'static [(f32, ConfigValidator)] =
    &[(0.0, self::runner_validator::runner_validator)];

pub fn validate_config(mut config: config::Config) -> Option<config::Config> {
    for (supported_version, validator_fn) in CONFIG_VALIDATORS {
        if &config.version >= supported_version {
            continue;
        }
        if let Some(c) = validator_fn(config) {
            config = c;
        } else {
            return None;
        }
    }
    Some(config)
}
