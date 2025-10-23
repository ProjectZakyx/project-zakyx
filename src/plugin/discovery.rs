// 🔍 PLUGIN DISCOVERY
// Automatisches Erkennen und Laden von Plugin-Manifesten
// Copyright © 2024 ZAKYX Browser Team

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use crate::plugin::types::{PluginManifest, PluginInfo, PluginConfig};
use crate::plugin::validator::PluginValidator;
use crate::error::ZAKYXBrowserResult;

/// Plugin-Discovery-Engine
#[derive(Debug)]
pub struct PluginDiscovery {
    config: PluginConfig,
    validator: PluginValidator,
}

impl PluginDiscovery {
    pub fn new(config: PluginConfig) -> Self {
        Self {
            validator: PluginValidator::new(config.validate_permissions),
            config,
        }
    }
    
    /// Entdeckt alle verfügbaren Plugins im Plugin-Verzeichnis
    pub fn discover_plugins(&self) -> ZAKYXBrowserResult<HashMap<String, PluginInfo>> {
        let mut plugins = HashMap::new();
        
        println!("🔍 Discovering plugins in: {:?}", self.config.plugins_directory);
        
        if !self.config.plugins_directory.exists() {
            println!("📁 Plugin directory doesn't exist, creating it...");
            std::fs::create_dir_all(&self.config.plugins_directory)
                .map_err(|e| crate::error::ZAKYXBrowserError::FileNotFound {
                    path: self.config.plugins_directory.to_string_lossy().to_string(),
                    operation: format!("create directory: {}", e),
                })?;
            return Ok(plugins);
        }
        
        let entries = fs::read_dir(&self.config.plugins_directory)
            .map_err(|e| crate::error::ZAKYXBrowserError::Config {
                message: format!("Failed to read plugins directory: {}", e),
                field: Some("plugins_directory".to_string()),
                fix_suggestion: Some("Check directory permissions".to_string()),
            })?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                match self.load_plugin_from_directory(&path) {
                    Ok(plugin_info) => {
                        let plugin_id = plugin_info.id.clone();
                        plugins.insert(plugin_id.clone(), plugin_info);
                        println!("📦 Discovered plugin: {}", plugin_id);
                    }
                    Err(e) => {
                        println!("⚠️ Failed to load plugin from {:?}: {}", path, e);
                    }
                }
            }
        }
        
        println!("✅ Discovery complete: {} plugins found", plugins.len());
        Ok(plugins)
    }
    
    /// Lädt ein Plugin aus einem Verzeichnis
    pub fn load_plugin_from_directory(&self, plugin_path: &Path) -> Result<PluginInfo, String> {
        let manifest_path = plugin_path.join("plugin.json");
        
        if !manifest_path.exists() {
            return Err("No plugin.json found".to_string());
        }
        
        let manifest = self.load_plugin_manifest(&manifest_path)?;
        
        // Validiere das Manifest
        if self.config.validate_permissions {
            let validation_result = self.validator.validate_manifest(&manifest);
            if !validation_result.is_valid() {
                return Err(format!("Invalid plugin manifest: {}", validation_result.errors.join("; ")));
            }
            
            // Zeige Warnungen an
            for warning in validation_result.warnings {
                println!("⚠️ Plugin warning: {}", warning);
            }
        }
        
        let plugin_id = self.extract_plugin_id(&plugin_path, &manifest)?;
        
        let plugin_info = PluginInfo {
            id: plugin_id,
            manifest,
            path: plugin_path.to_path_buf(),
            loaded: false,
            error: None,
            load_error: None,
        };
        
        Ok(plugin_info)
    }
    
    /// Lädt ein Plugin-Manifest aus einer Datei
    fn load_plugin_manifest(&self, manifest_path: &Path) -> Result<PluginManifest, String> {
        let manifest_content = fs::read_to_string(manifest_path)
            .map_err(|e| format!("Failed to read plugin.json: {}", e))?;
        
        let manifest: PluginManifest = serde_json::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse plugin.json: {}", e))?;
        
        Ok(manifest)
    }
    
    /// Extrahiert die Plugin-ID aus dem Verzeichnisnamen oder Manifest
    fn extract_plugin_id(&self, plugin_path: &Path, manifest: &PluginManifest) -> Result<String, String> {
        // Bevorzuge ID aus dem Manifest
        if let Some(id) = &manifest.id {
            if !id.trim().is_empty() {
                return Ok(id.clone());
            }
        }
        
        // Fallback: Verzeichnisname
        plugin_path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
            .ok_or_else(|| "Could not determine plugin ID".to_string())
    }
    
    /// Speichert ein Plugin-Manifest in eine Datei
    pub fn save_plugin_manifest(&self, plugin_info: &PluginInfo) -> Result<(), String> {
        let manifest_path = plugin_info.path.join("plugin.json");
        
        let manifest_content = serde_json::to_string_pretty(&plugin_info.manifest)
            .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
        
        fs::write(&manifest_path, manifest_content)
            .map_err(|e| format!("Failed to save manifest: {}", e))?;
        
        println!("💾 Saved plugin manifest: {}", plugin_info.id);
        Ok(())
    }
    
    /// Erstellt ein Plugin-Verzeichnis mit grundlegenden Dateien
    pub fn create_plugin_directory(&self, plugin_id: &str, manifest: &PluginManifest) -> Result<PathBuf, String> {
        let plugin_dir = self.config.plugins_directory.join(plugin_id);
        
        if plugin_dir.exists() {
            return Err(format!("Plugin directory already exists: {}", plugin_id));
        }
        
        fs::create_dir_all(&plugin_dir)
            .map_err(|e| format!("Failed to create plugin directory: {}", e))?;
        
        // Erstelle plugin.json
        let manifest_path = plugin_dir.join("plugin.json");
        let manifest_content = serde_json::to_string_pretty(manifest)
            .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
        
        fs::write(&manifest_path, manifest_content)
            .map_err(|e| format!("Failed to create plugin.json: {}", e))?;
        
        // Erstelle Hauptskript-Datei
        let main_script_path = plugin_dir.join(&manifest.main_script);
        fs::write(&main_script_path, "// Plugin main script\nconsole.log('Plugin loaded');\n")
            .map_err(|e| format!("Failed to create main script: {}", e))?;
        
        println!("📁 Created plugin directory: {}", plugin_id);
        Ok(plugin_dir)
    }
    
    /// Scannt nach Plugin-Updates
    pub fn scan_for_updates(&self, current_plugins: &HashMap<String, PluginInfo>) -> Result<Vec<String>, String> {
        let mut updated_plugins = Vec::new();
        
        for (plugin_id, plugin_info) in current_plugins {
            let manifest_path = plugin_info.path.join("plugin.json");
            
            if !manifest_path.exists() {
                continue;
            }
            
            match self.load_plugin_manifest(&manifest_path) {
                Ok(new_manifest) => {
                    if new_manifest.version != plugin_info.manifest.version {
                        updated_plugins.push(plugin_id.clone());
                        println!("🔄 Plugin update detected: {} ({} -> {})", 
                               plugin_id, plugin_info.manifest.version, new_manifest.version);
                    }
                }
                Err(e) => {
                    println!("⚠️ Failed to check for updates in {}: {}", plugin_id, e);
                }
            }
        }
        
        Ok(updated_plugins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    
    fn create_test_plugin_dir() -> PathBuf {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let test_dir = std::env::temp_dir().join(format!("zakyx_test_discovery_{}_{}", std::process::id(), nanos));
        if test_dir.exists() {
            let _ = fs::remove_dir_all(&test_dir);
        }
        fs::create_dir_all(&test_dir).unwrap();
        test_dir
    }
    
    fn create_test_manifest() -> PluginManifest {
        PluginManifest {
            id: Some("test-plugin".to_string()),
            name: "Test Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test Author".to_string(),
            homepage: None,
            main_script: "main.js".to_string(),
            permissions: vec!["network".to_string()],
            api_version: "1.0".to_string(),
            enabled: true,
            background: None,
            content_scripts: None,
            web_accessible_resources: None,
            browser_action: None,
            options_page: None,
            manifest_version: None,
            minimum_zakyx_version: None,
            dependencies: None,
            settings: None,
            update_url: None,
        }
    }
    
    #[test]
    fn test_plugin_discovery_creation() {
        let config = PluginConfig::default();
        let discovery = PluginDiscovery::new(config);
        
        assert!(discovery.validator.validate_plugin_permissions(&create_test_manifest()).is_ok());
    }
    
    #[test]
    fn test_discover_plugins_empty_directory() {
        let test_dir = create_test_plugin_dir();
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            ..Default::default()
        };
        
        let discovery = PluginDiscovery::new(config);
        let plugins = discovery.discover_plugins().unwrap();
        
        assert!(plugins.is_empty());
        
        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
    
    #[test]
    fn test_load_plugin_from_directory() {
        let test_dir = create_test_plugin_dir();
        let plugin_dir = test_dir.join("test-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        
        let manifest = create_test_manifest();
        let manifest_content = serde_json::to_string_pretty(&manifest).unwrap();
        fs::write(plugin_dir.join("plugin.json"), manifest_content).unwrap();
        fs::write(plugin_dir.join("main.js"), "// Test script").unwrap();
        
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            ..Default::default()
        };
        
        let discovery = PluginDiscovery::new(config);
        let plugin_info = discovery.load_plugin_from_directory(&plugin_dir).unwrap();
        
        assert_eq!(plugin_info.id, "test-plugin");
        assert_eq!(plugin_info.manifest.name, "Test Plugin");
        assert!(!plugin_info.loaded);
        
        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
    
    #[test]
    fn test_create_plugin_directory() {
        let test_dir = create_test_plugin_dir();
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            ..Default::default()
        };
        
        let discovery = PluginDiscovery::new(config);
        let manifest = create_test_manifest();
        
        let plugin_dir = discovery.create_plugin_directory("new-plugin", &manifest).unwrap();
        
        assert!(plugin_dir.exists());
        assert!(plugin_dir.join("plugin.json").exists());
        assert!(plugin_dir.join("main.js").exists());
        
        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
    
    #[test]
    fn test_scan_for_updates() {
        let test_dir = create_test_plugin_dir();
        let plugin_dir = test_dir.join("test-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        
        let manifest = create_test_manifest();
        let manifest_content = serde_json::to_string_pretty(&manifest).unwrap();
        fs::write(plugin_dir.join("plugin.json"), manifest_content).unwrap();
        
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            ..Default::default()
        };
        
        let discovery = PluginDiscovery::new(config);
        
        // Erstelle Plugin-Info mit alter Version
        let mut plugin_info = PluginInfo {
            id: "test-plugin".to_string(),
            manifest: manifest.clone(),
            path: plugin_dir.clone(),
            loaded: false,
            error: None,
        };
        plugin_info.manifest.version = "0.9.0".to_string();
        
        let mut current_plugins = HashMap::new();
        current_plugins.insert("test-plugin".to_string(), plugin_info);
        
        let updates = discovery.scan_for_updates(&current_plugins).unwrap();
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0], "test-plugin");
        
        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
} 
