// conf.rs serialize configuration file

use std::error::Error;
use std::fs;

use serde::{Deserialize, Serialize};
use toml;

// Define the configuration struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub model: String,
}
#[allow(dead_code)]
// Read the configuration file if not found create a default configuration file
pub fn read_config_or_default(file: &str) -> Result<Config, Box<dyn Error>> {
    let config = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(_) => {
            let default_config = Config {
                host: "localhost".to_string(),
                model: "llama2".to_string(),
                port: 11434,
            };
            fs::write(file, toml::to_string(&default_config)?)?;
            return Ok(default_config);
        }
    };
    let config: Config = toml::from_str(&config)?;
    Ok(config)
}

#[allow(dead_code)]
// Write the configuration file
pub fn write_config(file: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    let toml = toml::to_string(config)?;
    fs::write(file, toml)?;
    Ok(())
}
// Path: src/fileop.rs

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_read_write_config() {
        let file = "test_config.toml";
        let config = Config {
            host: "localhost".to_string(),
            port: 11434,
            model: "llama2".to_string(),
        };

        // Write the configuration to the file
        write_config(file, &config).unwrap();

        // Read the configuration from the file
        let read_config = read_config_or_default(file).unwrap();

        // Check if the read configuration is the same as the written configuration
        assert_eq!(config.host, read_config.host);
        assert_eq!(config.port, read_config.port);

        // Clean up the test file
        if Path::new(file).exists() {
            fs::remove_file(file).unwrap();
        }
    }
}
