use crate::{
    configuration::Configuration,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

pub struct JsonDeserializer {}

impl JsonDeserializer {
    pub fn new() -> Self {
        JsonDeserializer {}
    }
}

impl Default for JsonDeserializer {
    fn default() -> Self {
        JsonDeserializer::new()
    }
}

impl Format for JsonDeserializer {
    fn transform(&self, input: Vec<u8>) -> Result<Configuration, ConfigurationError> {
        serde_json::from_slice::<Configuration>(&input)
            .map_err(|e| ErrorCode::SerdeError(e.to_string()).into())
    }
}
