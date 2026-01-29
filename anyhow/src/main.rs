use std::{error::Error, fs};

use anyhow::Context;

fn get_config_content() -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string("config.txt")
        .map_err(|e| format!("Failed to read config.txt: {}", e))?;

    let parsed: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(parsed.to_string())
}

fn get_config_content_with_anyhow() -> anyhow::Result<String> {
    let content = fs::read_to_string("config.txt").context("Failed to read config.txt")?;
    let parsed: serde_json::Value =
        serde_json::from_str(&content).context("Failed to config as JSON")?;

    Ok(parsed.to_string())
}

fn main() {
    match get_config_content() {
        Ok(config) => println!("Config content: {}", config),
        Err(e) => eprintln!("Error: {}", e),
    }

    match get_config_content_with_anyhow() {
        Ok(config) => println!("Config content: {}", config),
        Err(e) => eprintln!("Error: {}", e),
    }
}
