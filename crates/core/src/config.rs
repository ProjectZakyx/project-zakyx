use crate::browser::BrowserSettings;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub browser: BrowserSettings,
    pub history: Vec<String>,
    pub bookmarks: Vec<Bookmark>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bookmark {
    pub title: String,
    pub url: String,
    pub folder: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            browser: BrowserSettings::default(),
            history: Vec::new(),
            bookmarks: Vec::new(),
        }
    }
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    if !Path::new(&config_path).exists() {
        return Ok(Config::default());
    }
    
    let content = fs::read_to_string(&config_path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;
    let content = serde_json::to_string_pretty(config)?;
    fs::write(&config_path, content)?;
    Ok(())
}

fn get_config_path() -> Result<String> {
    let root = std::env::current_dir()?;
    let config_path = root.join("config.json");
    Ok(config_path.display().to_string())
}
