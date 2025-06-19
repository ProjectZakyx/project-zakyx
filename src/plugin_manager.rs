// 🔌 PLUGIN MANAGER
// Dynamisches Laden und Verwalten von Browser-Plugins
// Copyright © 2024 Ora Browser Team

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::fs;


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
    pub dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub settings: Option<serde_json::Value>,
    #[serde(default)]
    pub update_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginBackground {
    pub scripts: Vec<String>,
    #[serde(default)]
    pub persistent: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContentScript {
    pub matches: Vec<String>,
    pub js: Vec<String>,
    #[serde(default)]
    pub run_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginBrowserAction {
    #[serde(default)]
    pub default_title: Option<String>,
    #[serde(default)]
    pub default_popup: Option<String>,
    #[serde(default)]
    pub default_icon: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub manifest: PluginManifest,
    pub path: PathBuf,
    pub loaded: bool,
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct PluginManager {
    plugins: HashMap<String, PluginInfo>,
    plugins_directory: PathBuf,
}

impl PluginManager {
    pub fn new() -> Self {
        let plugins_dir = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("extensions");
        
        Self {
            plugins: HashMap::new(),
            plugins_directory: plugins_dir,
        }
    }
    
    pub fn initialize(&mut self) -> Result<(), String> {
        println!("🔌 Initializing Plugin Manager...");
        
        // Erstelle Plugin-Verzeichnis falls nicht vorhanden
        if !self.plugins_directory.exists() {
            if let Err(e) = fs::create_dir_all(&self.plugins_directory) {
                return Err(format!("Failed to create plugins directory: {}", e));
            }
        }
        
        // Lade alle verfügbaren Plugins
        self.discover_plugins()?;
        
        // Lade aktivierte Plugins
        self.load_enabled_plugins()?;
        
        println!("✅ Plugin Manager initialized with {} plugins", self.plugins.len());
        Ok(())
    }
    
    fn discover_plugins(&mut self) -> Result<(), String> {
        println!("🔍 Discovering plugins in: {:?}", self.plugins_directory);
        
        if !self.plugins_directory.exists() {
            return Ok(());
        }
        
        let entries = fs::read_dir(&self.plugins_directory)
            .map_err(|e| format!("Failed to read plugins directory: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Err(e) = self.load_plugin_manifest(&path) {
                    println!("⚠️ Failed to load plugin from {:?}: {}", path, e);
                }
            }
        }
        
        Ok(())
    }
    
    fn load_plugin_manifest(&mut self, plugin_path: &PathBuf) -> Result<(), String> {
        let manifest_path = plugin_path.join("plugin.json");
        
        if !manifest_path.exists() {
            return Err("No plugin.json found".to_string());
        }
        
        let manifest_content = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read plugin.json: {}", e))?;
        
        let manifest: PluginManifest = serde_json::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse plugin.json: {}", e))?;
        
        let plugin_id = plugin_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        let plugin_info = PluginInfo {
            id: plugin_id.clone(),
            manifest,
            path: plugin_path.clone(),
            loaded: false,
            error: None,
        };
        
        self.plugins.insert(plugin_id.clone(), plugin_info);
        println!("📦 Discovered plugin: {}", plugin_id);
        
        Ok(())
    }
    
    fn load_enabled_plugins(&mut self) -> Result<(), String> {
        let enabled_plugins: Vec<String> = self.plugins
            .iter()
            .filter(|(_, info)| info.manifest.enabled)
            .map(|(id, _)| id.clone())
            .collect();
        
        for plugin_id in enabled_plugins {
            if let Err(e) = self.load_plugin(&plugin_id) {
                println!("❌ Failed to load plugin {}: {}", plugin_id, e);
                if let Some(plugin) = self.plugins.get_mut(&plugin_id) {
                    plugin.error = Some(e);
                }
            }
        }
        
        Ok(())
    }
    
    pub fn load_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        // Validiere Plugin-Berechtigungen zuerst
        {
            let plugin = self.plugins.get(plugin_id)
                .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
            
            if plugin.loaded {
                return Ok(());
            }
            
            self.validate_plugin_permissions(&plugin.manifest)?;
            
            // Lade Plugin-Script
            let script_path = plugin.path.join(&plugin.manifest.main_script);
            if !script_path.exists() {
                return Err(format!("Main script not found: {}", plugin.manifest.main_script));
            }
        }
        
        // Jetzt können wir sicher mutieren
        let plugin = self.plugins.get_mut(plugin_id).unwrap();
        
        println!("🔄 Loading plugin: {}", plugin_id);
        
        // Hier würde normalerweise das JavaScript/Plugin-Script geladen werden
        // Für jetzt markieren wir es als geladen
        plugin.loaded = true;
        plugin.error = None;
        
        println!("✅ Plugin loaded: {} v{}", plugin.manifest.name, plugin.manifest.version);
        Ok(())
    }
    
    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let plugin = self.plugins.get_mut(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
        
        if !plugin.loaded {
            return Ok(());
        }
        
        println!("🔄 Unloading plugin: {}", plugin_id);
        
        // Hier würde normalerweise das Plugin-Script entladen werden
        plugin.loaded = false;
        
        println!("✅ Plugin unloaded: {}", plugin.manifest.name);
        Ok(())
    }
    
    pub fn enable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        // Erst validieren und Manifest speichern
        {
            let plugin = self.plugins.get_mut(plugin_id)
                .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
            
            plugin.manifest.enabled = true;
        }
        
        self.save_plugin_manifest(plugin_id)?;
        
        // Dann laden falls nötig
        let should_load = {
            let plugin = self.plugins.get(plugin_id).unwrap();
            !plugin.loaded
        };
        
        if should_load {
            self.load_plugin(plugin_id)?;
        }
        
        Ok(())
    }
    
    pub fn disable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        // Erst validieren und Manifest speichern
        let should_unload = {
            let plugin = self.plugins.get_mut(plugin_id)
                .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
            
            plugin.manifest.enabled = false;
            plugin.loaded
        };
        
        self.save_plugin_manifest(plugin_id)?;
        
        // Dann entladen falls nötig
        if should_unload {
            self.unload_plugin(plugin_id)?;
        }
        
        Ok(())
    }
    
    fn save_plugin_manifest(&self, plugin_id: &str) -> Result<(), String> {
        let plugin = self.plugins.get(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
        
        let manifest_path = plugin.path.join("plugin.json");
        let manifest_content = serde_json::to_string_pretty(&plugin.manifest)
            .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
        
        fs::write(&manifest_path, manifest_content)
            .map_err(|e| format!("Failed to save manifest: {}", e))?;
        
        Ok(())
    }
    
    fn validate_plugin_permissions(&self, manifest: &PluginManifest) -> Result<(), String> {
        // Validiere API-Version
        if manifest.api_version != "1.0" {
            return Err(format!("Unsupported API version: {}", manifest.api_version));
        }
        
        // Validiere Berechtigungen
        let allowed_permissions = [
            "network", "storage", "tabs", "bookmarks", "history", "settings",
            "webRequest", "proxy", "activeTab", "background"
        ];
        
        for permission in &manifest.permissions {
            if !allowed_permissions.contains(&permission.as_str()) {
                return Err(format!("Unknown permission: {}", permission));
            }
        }
        
        Ok(())
    }
    
    pub fn get_all_plugins(&self) -> Vec<&PluginInfo> {
        self.plugins.values().collect()
    }
    
    pub fn get_loaded_plugins(&self) -> Vec<&PluginInfo> {
        self.plugins.values().filter(|p| p.loaded).collect()
    }
    
    #[allow(dead_code)]
    pub fn get_plugin(&self, plugin_id: &str) -> Option<&PluginInfo> {
        self.plugins.get(plugin_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    
    fn create_test_plugin_dir() -> PathBuf {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let test_dir = std::env::temp_dir().join(format!("ora_test_plugins_{}_{}", std::process::id(), nanos));
        if test_dir.exists() {
            let _ = fs::remove_dir_all(&test_dir);
        }
        fs::create_dir_all(&test_dir).unwrap();
        test_dir
    }
    
    fn create_test_plugin_manifest(dir: &Path, name: &str, enabled: bool) {
        let plugin_dir = dir.join(name);
        fs::create_dir_all(&plugin_dir).unwrap();
        
        let manifest = PluginManifest {
            id: Some(name.to_string()),
            name: name.to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test Author".to_string(),
            homepage: None,
            main_script: "main.js".to_string(),
            permissions: vec!["network".to_string()],
            api_version: "1.0".to_string(),
            enabled,
            background: None,
            content_scripts: None,
            web_accessible_resources: None,
            browser_action: None,
            options_page: None,
            manifest_version: None,
            minimum_ora_version: None,
            dependencies: None,
            settings: None,
            update_url: None,
        };
        
        let manifest_content = serde_json::to_string_pretty(&manifest).unwrap();
        fs::write(plugin_dir.join("plugin.json"), manifest_content).unwrap();
        fs::write(plugin_dir.join("main.js"), "// Test plugin script").unwrap();
    }
    
    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert!(manager.plugins.is_empty());
        assert!(manager.plugins_directory.ends_with("extensions"));
    }
    
    #[test]
    fn test_plugin_discovery() {
        let test_dir = create_test_plugin_dir();
        create_test_plugin_manifest(&test_dir, "test-plugin", true);
        
        let mut manager = PluginManager::new();
        manager.plugins_directory = test_dir.clone();
        
        assert!(manager.discover_plugins().is_ok());
        assert_eq!(manager.plugins.len(), 1);
        assert!(manager.plugins.contains_key("test-plugin"));
        
        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
    
    #[test]
    fn test_plugin_manifest_validation() {
        let manager = PluginManager::new();
        
        // Valid manifest
        let valid_manifest = PluginManifest {
            id: Some("test-plugin".to_string()),
            name: "Test Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            author: "Test".to_string(),
            homepage: None,
            main_script: "main.js".to_string(),
            permissions: vec!["network".to_string(), "storage".to_string()],
            api_version: "1.0".to_string(),
            enabled: true,
            background: None,
            content_scripts: None,
            web_accessible_resources: None,
            browser_action: None,
            options_page: None,
            manifest_version: None,
            minimum_ora_version: None,
            dependencies: None,
            settings: None,
            update_url: None,
        };
        
        assert!(manager.validate_plugin_permissions(&valid_manifest).is_ok());
        
        // Invalid API version
        let invalid_api_manifest = PluginManifest {
            api_version: "2.0".to_string(),
            ..valid_manifest.clone()
        };
        
        assert!(manager.validate_plugin_permissions(&invalid_api_manifest).is_err());
        
        // Invalid permission
        let invalid_perm_manifest = PluginManifest {
            permissions: vec!["invalid_permission".to_string()],
            ..valid_manifest
        };
        
        assert!(manager.validate_plugin_permissions(&invalid_perm_manifest).is_err());
    }
    
    #[test]
    fn test_plugin_enable_disable() {
        let test_dir = create_test_plugin_dir();
        create_test_plugin_manifest(&test_dir, "test-plugin", false);
        
        let mut manager = PluginManager::new();
        manager.plugins_directory = test_dir.clone();
        let _ = manager.discover_plugins();
        
        // Test enable using the public methods
        assert!(manager.enable_plugin("test-plugin").is_ok());
        if let Some(plugin) = manager.plugins.get("test-plugin") {
            assert!(plugin.manifest.enabled);
        }
        
        // Test disable using the public methods
        assert!(manager.disable_plugin("test-plugin").is_ok());
        if let Some(plugin) = manager.plugins.get("test-plugin") {
            assert!(!plugin.manifest.enabled);
        }
        
        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
    
    #[test]
    fn test_plugin_load_unload() {
        let test_dir = create_test_plugin_dir();
        create_test_plugin_manifest(&test_dir, "test-plugin", true);
        
        let mut manager = PluginManager::new();
        manager.plugins_directory = test_dir.clone();
        manager.discover_plugins().unwrap();
        
        // Test load
        assert!(manager.load_plugin("test-plugin").is_ok());
        let plugin = manager.plugins.get("test-plugin").unwrap();
        assert!(plugin.loaded);
        assert!(plugin.error.is_none());
        
        // Test unload
        assert!(manager.unload_plugin("test-plugin").is_ok());
        let plugin = manager.plugins.get("test-plugin").unwrap();
        assert!(!plugin.loaded);
        
        // Cleanup - ignore errors
        let _ = fs::remove_dir_all(&test_dir);
    }
    
    #[test]
    fn test_plugin_queries() {
        let test_dir = create_test_plugin_dir();
        create_test_plugin_manifest(&test_dir, "enabled-plugin", true);
        create_test_plugin_manifest(&test_dir, "disabled-plugin", false);
        
        let mut manager = PluginManager::new();
        manager.plugins_directory = test_dir.clone();
        let _ = manager.discover_plugins();
        let _ = manager.load_enabled_plugins();
        
        // Test get_all_plugins
        let all_plugins = manager.get_all_plugins();
        assert_eq!(all_plugins.len(), 2);
        
        // Test get_loaded_plugins (only enabled plugin should be loaded)
        let loaded_plugins = manager.get_loaded_plugins();
        assert_eq!(loaded_plugins.len(), 1);
        assert_eq!(loaded_plugins[0].id, "enabled-plugin");
        
        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
}
