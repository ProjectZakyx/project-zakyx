// 🔌 PLUGIN TYPES
// Gemeinsame Datenstrukturen für das Plugin-System
// Copyright © 2024 Ora Browser Team

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::error::{OraBrowserError, OraBrowserResult};

/// Plugin-Manifest mit allen Konfigurationsoptionen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    #[serde(default)]
    pub id: Option<String>,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    #[serde(default)]
    pub homepage: Option<String>,
    pub main_script: String,
    pub permissions: Vec<String>,
    pub api_version: String,
    pub enabled: bool,
    #[serde(default)]
    pub background: Option<PluginBackground>,
    #[serde(default)]
    pub content_scripts: Option<Vec<PluginContentScript>>,
    #[serde(default)]
    pub web_accessible_resources: Option<Vec<String>>,
    #[serde(default)]
    pub browser_action: Option<PluginBrowserAction>,
    #[serde(default)]
    pub options_page: Option<String>,
    #[serde(default)]
    pub manifest_version: Option<u32>,
    #[serde(default)]
    pub minimum_ora_version: Option<String>,
    #[serde(default)]
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(default)]
    pub settings: Option<serde_json::Value>,
    #[serde(default)]
    pub update_url: Option<String>,
}

/// Plugin-Background-Konfiguration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginBackground {
    pub scripts: Vec<String>,
    #[serde(default)]
    pub persistent: Option<bool>,
}

/// Plugin-Content-Script-Konfiguration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContentScript {
    pub matches: Vec<String>,
    pub js: Vec<String>,
    #[serde(default)]
    pub run_at: Option<String>,
}

/// Plugin-Browser-Action-Konfiguration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginBrowserAction {
    #[serde(default)]
    pub default_title: Option<String>,
    #[serde(default)]
    pub default_popup: Option<String>,
    #[serde(default)]
    pub default_icon: Option<HashMap<String, String>>,
}

/// Plugin-Information und -Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub manifest: PluginManifest,
    pub path: PathBuf,
    pub loaded: bool,
    pub error: Option<String>,
}

/// Plugin-Status für Statistiken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginStats {
    pub total_plugins: usize,
    pub loaded_plugins: usize,
    pub enabled_plugins: usize,
    pub failed_plugins: usize,
}

/// Plugin-Konfiguration für Manager
#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub plugins_directory: PathBuf,
    pub auto_load_enabled: bool,
    pub validate_permissions: bool,
    pub allow_dev_plugins: bool,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            plugins_directory: std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("extensions"),
            auto_load_enabled: true,
            validate_permissions: true,
            allow_dev_plugins: false,
        }
    }
}

/// Erlaubte Plugin-Berechtigungen
pub const ALLOWED_PERMISSIONS: &[&str] = &[
    "network",
    "storage", 
    "tabs",
    "bookmarks",
    "history",
    "settings",
    "webRequest",
    "proxy",
    "activeTab",
    "background",
];

/// Unterstützte API-Versionen
pub const SUPPORTED_API_VERSIONS: &[&str] = &["1.0"];

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_config_default() {
        let config = PluginConfig::default();
        assert!(config.plugins_directory.ends_with("extensions"));
        assert!(config.auto_load_enabled);
        assert!(config.validate_permissions);
        assert!(!config.allow_dev_plugins);
    }
    
    #[test]
    fn test_plugin_stats_calculation() {
        let stats = PluginStats {
            total_plugins: 5,
            loaded_plugins: 3,
            enabled_plugins: 4,
            failed_plugins: 1,
        };
        
        assert_eq!(stats.total_plugins, 5);
        assert_eq!(stats.loaded_plugins, 3);
        assert_eq!(stats.enabled_plugins, 4);
        assert_eq!(stats.failed_plugins, 1);
    }
    
    #[test]
    fn test_allowed_permissions() {
        assert!(ALLOWED_PERMISSIONS.contains(&"network"));
        assert!(ALLOWED_PERMISSIONS.contains(&"storage"));
        assert!(ALLOWED_PERMISSIONS.contains(&"tabs"));
        assert!(!ALLOWED_PERMISSIONS.contains(&"invalid_permission"));
    }
} 