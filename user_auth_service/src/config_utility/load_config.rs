use std::env;
use std::error::Error;
use std::fs;
use toml;

use super::config_loader::ConfigLoader;

pub fn load_config() -> Result<ConfigLoader, Box<dyn Error>> {
    let env = env::var("APP_ENV").unwrap_or_else(|_| "development".into());

    let current_directory = match env::current_dir() {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return Err(Box::<dyn Error>::from("Failed to load config"));
        }
    };

    let config_path = match env.as_str() {
        "development" => format!("{}/.config/development_config.toml", current_directory),
        "production" => format!("{}/.config/production_config.toml", current_directory),
        _ => format!("{}/.config/development_config.toml", current_directory),
    };

    let content = fs::read_to_string(config_path)?;
    let loader: ConfigLoader = toml::from_str(&content)?;

    Ok(loader)
}
