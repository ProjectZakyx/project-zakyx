// 🔌 PLUGIN MANAGER
// Hauptkoordinator für alle Plugin-Operationen
// Copyright © 2024 Ora Browser Team

use std::collections::HashMap;
use crate::plugin::types::{PluginInfo, PluginConfig, PluginStats};
use crate::plugin::discovery::PluginDiscovery;
use crate::plugin::loader::PluginLoader;

/// Hauptklasse für Plugin-Management
#[derive(Debug)]
pub struct PluginManager {
    plugins: HashMap<String, PluginInfo>,
    discovery: PluginDiscovery,
    loader: PluginLoader,
    config: PluginConfig,
}

impl PluginManager {
    /// Erstellt einen neuen Plugin-Manager
    pub fn new() -> Self {
        let config = PluginConfig::default();
        let discovery = PluginDiscovery::new(config.clone());
        let loader = PluginLoader::new(config.clone());
        
        Self {
            plugins: HashMap::new(),
            discovery,
            loader,
            config,
        }
    }
    
    /// Erstellt einen neuen Plugin-Manager mit benutzerdefinierter Konfiguration
    pub fn with_config(config: PluginConfig) -> Self {
        let discovery = PluginDiscovery::new(config.clone());
        let loader = PluginLoader::new(config.clone());
        
        Self {
            plugins: HashMap::new(),
            discovery,
            loader,
            config,
        }
    }
    
    /// Initialisiert den Plugin-Manager
    pub fn initialize(&mut self) -> Result<(), String> {
        println!("🔌 Initializing Plugin Manager...");
        
        // Erstelle Plugin-Verzeichnis falls nicht vorhanden
        if !self.config.plugins_directory.exists() {
            std::fs::create_dir_all(&self.config.plugins_directory)
                .map_err(|e| format!("Failed to create plugins directory: {}", e))?;
        }
        
        // Entdecke alle verfügbaren Plugins
        self.discover_plugins()?;
        
        // Lade aktivierte Plugins automatisch
        if self.config.auto_load_enabled {
            self.load_enabled_plugins()?;
        }
        
        println!("✅ Plugin Manager initialized with {} plugins", self.plugins.len());
        Ok(())
    }
    
    /// Entdeckt alle verfügbaren Plugins
    pub fn discover_plugins(&mut self) -> Result<(), String> {
        self.plugins = self.discovery.discover_plugins()
            .map_err(|e| e.user_message())?;
        Ok(())
    }
    
    /// Lädt alle aktivierten Plugins
    pub fn load_enabled_plugins(&mut self) -> Result<(), String> {
        self.loader.load_enabled_plugins(&mut self.plugins)
    }
    
    /// Lädt ein einzelnes Plugin
    pub fn load_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        self.loader.load_plugin(plugin_id, &mut self.plugins)
    }
    
    /// Entlädt ein Plugin
    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        self.loader.unload_plugin(plugin_id, &mut self.plugins)
    }
    
    /// Aktiviert ein Plugin
    pub fn enable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        // Validiere Plugin-Existenz
        let plugin = self.plugins.get_mut(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
        
        // Aktiviere Plugin
        plugin.manifest.enabled = true;
        
        // Speichere Änderungen
        self.discovery.save_plugin_manifest(plugin)?;
        
        // Lade Plugin falls konfiguriert
        if self.config.auto_load_enabled && !plugin.loaded {
            self.load_plugin(plugin_id)?;
        }
        
        println!("✅ Plugin enabled: {}", plugin_id);
        Ok(())
    }
    
    /// Deaktiviert ein Plugin
    pub fn disable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        // Validiere Plugin-Existenz
        let should_unload = {
            let plugin = self.plugins.get_mut(plugin_id)
                .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
            
            // Deaktiviere Plugin
            plugin.manifest.enabled = false;
            
            // Speichere Änderungen
            self.discovery.save_plugin_manifest(plugin)?;
            
            plugin.loaded
        };
        
        // Entlade Plugin falls nötig
        if should_unload {
            self.unload_plugin(plugin_id)?;
        }
        
        println!("✅ Plugin disabled: {}", plugin_id);
        Ok(())
    }
    
    /// Neu lädt ein Plugin
    pub fn reload_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        self.loader.reload_plugin(plugin_id, &mut self.plugins)
    }
    
    /// Installiert ein neues Plugin
    pub fn install_plugin(&mut self, plugin_path: &std::path::Path) -> Result<String, String> {
        // Lade Plugin aus Verzeichnis
        let plugin_info = self.discovery.load_plugin_from_directory(plugin_path)?;
        let plugin_id = plugin_info.id.clone();
        
        // Prüfe ob Plugin bereits existiert
        if self.plugins.contains_key(&plugin_id) {
            return Err(format!("Plugin {} already exists", plugin_id));
        }
        
        // Füge Plugin hinzu
        self.plugins.insert(plugin_id.clone(), plugin_info);
        
        println!("📦 Plugin installed: {}", plugin_id);
        Ok(plugin_id)
    }
    
    /// Deinstalliert ein Plugin
    pub fn uninstall_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        // Prüfe Plugin-Existenz und sammle Informationen
        let (plugin_loaded, plugin_path) = {
            let plugin = self.plugins.get(plugin_id)
                .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
            (plugin.loaded, plugin.path.clone())
        };
        
        // Entlade Plugin falls geladen
        if plugin_loaded {
            self.unload_plugin(plugin_id)?;
        }
        
        // Entferne Plugin-Verzeichnis
        std::fs::remove_dir_all(&plugin_path)
            .map_err(|e| format!("Failed to remove plugin directory: {}", e))?;
        
        // Entferne Plugin aus Liste
        self.plugins.remove(plugin_id);
        
        println!("🗑️ Plugin uninstalled: {}", plugin_id);
        Ok(())
    }
    
    /// Aktualisiert ein Plugin
    pub fn update_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        // Prüfe Plugin-Existenz
        let plugin = self.plugins.get(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
        
        // Lade aktuelles Manifest
        let updated_plugin = self.discovery.load_plugin_from_directory(&plugin.path)?;
        
        // Prüfe ob Update verfügbar
        if updated_plugin.manifest.version == plugin.manifest.version {
            return Ok(()); // Keine Aktualisierung nötig
        }
        
        // Entlade Plugin falls geladen
        let was_loaded = plugin.loaded;
        if was_loaded {
            self.unload_plugin(plugin_id)?;
        }
        
        // Aktualisiere Plugin-Info
        self.plugins.insert(plugin_id.to_string(), updated_plugin);
        
        // Lade Plugin neu falls vorher geladen
        if was_loaded {
            self.load_plugin(plugin_id)?;
        }
        
        println!("🔄 Plugin updated: {}", plugin_id);
        Ok(())
    }
    
    /// Scannt nach Plugin-Updates
    pub fn scan_for_updates(&self) -> Result<Vec<String>, String> {
        self.discovery.scan_for_updates(&self.plugins)
    }
    
    /// Gibt alle Plugins zurück
    pub fn get_all_plugins(&self) -> Vec<&PluginInfo> {
        self.plugins.values().collect()
    }
    
    /// Gibt alle geladenen Plugins zurück
    pub fn get_loaded_plugins(&self) -> Vec<&PluginInfo> {
        self.plugins.values().filter(|p| p.loaded).collect()
    }
    
    /// Gibt alle aktivierten Plugins zurück
    pub fn get_enabled_plugins(&self) -> Vec<&PluginInfo> {
        self.plugins.values().filter(|p| p.manifest.enabled).collect()
    }
    
    /// Gibt ein spezifisches Plugin zurück
    pub fn get_plugin(&self, plugin_id: &str) -> Option<&PluginInfo> {
        self.plugins.get(plugin_id)
    }
    
    /// Gibt Plugin-Statistiken zurück
    pub fn get_plugin_stats(&self) -> PluginStats {
        let total_plugins = self.plugins.len();
        let loaded_plugins = self.plugins.values().filter(|p| p.loaded).count();
        let enabled_plugins = self.plugins.values().filter(|p| p.manifest.enabled).count();
        let failed_plugins = self.plugins.values().filter(|p| p.error.is_some()).count();
        
        PluginStats {
            total_plugins,
            loaded_plugins,
            enabled_plugins,
            failed_plugins,
        }
    }
    
    /// Gibt die Plugin-Konfiguration zurück
    pub fn get_config(&self) -> &PluginConfig {
        &self.config
    }
    
    /// Aktualisiert die Plugin-Konfiguration
    pub fn update_config(&mut self, config: PluginConfig) -> Result<(), String> {
        self.config = config.clone();
        self.discovery = PluginDiscovery::new(config.clone());
        self.loader = PluginLoader::new(config);
        
        println!("⚙️ Plugin configuration updated");
        Ok(())
    }
    
    /// Führt eine Bereinigung durch
    pub fn cleanup(&mut self) -> Result<(), String> {
        // Entlade alle Plugins
        self.loader.unload_all_plugins(&mut self.plugins)?;
        
        // Lösche temporäre Dateien
        // TODO: Implementiere temporäre Dateien-Bereinigung
        
        println!("🧹 Plugin cleanup completed");
        Ok(())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

// Legacy-Kompatibilität
impl PluginManager {
    /// Legacy-Methode für Validierung (für Rückwärtskompatibilität)
    pub fn validate_plugin_permissions(&self, plugin_id: &str) -> Result<(), String> {
        let plugin = self.plugins.get(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
        
        use crate::plugin::validator::PluginValidator;
        let validator = PluginValidator::new(self.config.validate_permissions);
        validator.validate_plugin_permissions(&plugin.manifest)
            .map_err(|e| e.user_message())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    
    fn create_test_plugin_dir() -> std::path::PathBuf {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let test_dir = std::env::temp_dir().join(format!("ora_test_manager_{}_{}", std::process::id(), nanos));
        if test_dir.exists() {
            let _ = fs::remove_dir_all(&test_dir);
        }
        fs::create_dir_all(&test_dir).unwrap();
        test_dir
    }
    
    fn create_test_manifest_file(plugin_dir: &std::path::Path, name: &str, enabled: bool) {
        use crate::plugin::types::PluginManifest;
        
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
        fs::write(plugin_dir.join("main.js"), "console.log('Test plugin');").unwrap();
    }
    
    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert!(manager.plugins.is_empty());
        assert!(manager.config.plugins_directory.ends_with("extensions"));
    }
    
    #[test]
    fn test_plugin_manager_initialization() {
        let test_dir = create_test_plugin_dir();
        let plugin_dir = test_dir.join("test-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        create_test_manifest_file(&plugin_dir, "test-plugin", true);
        
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            auto_load_enabled: false,
            ..Default::default()
        };
        
        let mut manager = PluginManager::with_config(config);
        assert!(manager.initialize().is_ok());
        assert_eq!(manager.plugins.len(), 1);
        
        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
    
    #[test]
    fn test_enable_disable_plugin() {
        let test_dir = create_test_plugin_dir();
        let plugin_dir = test_dir.join("test-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        create_test_manifest_file(&plugin_dir, "test-plugin", false);
        
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            auto_load_enabled: false,
            ..Default::default()
        };
        
        let mut manager = PluginManager::with_config(config);
        manager.initialize().unwrap();
        
        // Enable plugin
        assert!(manager.enable_plugin("test-plugin").is_ok());
        let plugin = manager.get_plugin("test-plugin").unwrap();
        assert!(plugin.manifest.enabled);
        
        // Disable plugin
        assert!(manager.disable_plugin("test-plugin").is_ok());
        let plugin = manager.get_plugin("test-plugin").unwrap();
        assert!(!plugin.manifest.enabled);
        
        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
    
    #[test]
    fn test_plugin_stats() {
        let test_dir = create_test_plugin_dir();
        let plugin_dir1 = test_dir.join("plugin1");
        let plugin_dir2 = test_dir.join("plugin2");
        fs::create_dir_all(&plugin_dir1).unwrap();
        fs::create_dir_all(&plugin_dir2).unwrap();
        create_test_manifest_file(&plugin_dir1, "plugin1", true);
        create_test_manifest_file(&plugin_dir2, "plugin2", false);
        
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            auto_load_enabled: false,
            ..Default::default()
        };
        
        let mut manager = PluginManager::with_config(config);
        manager.initialize().unwrap();
        
        let stats = manager.get_plugin_stats();
        assert_eq!(stats.total_plugins, 2);
        assert_eq!(stats.enabled_plugins, 1);
        assert_eq!(stats.loaded_plugins, 0);
        assert_eq!(stats.failed_plugins, 0);
        
        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
    
    #[test]
    fn test_load_unload_plugin() {
        let test_dir = create_test_plugin_dir();
        let plugin_dir = test_dir.join("test-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        create_test_manifest_file(&plugin_dir, "test-plugin", true);
        
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            auto_load_enabled: false,
            ..Default::default()
        };
        
        let mut manager = PluginManager::with_config(config);
        manager.initialize().unwrap();
        
        // Load plugin
        assert!(manager.load_plugin("test-plugin").is_ok());
        let plugin = manager.get_plugin("test-plugin").unwrap();
        assert!(plugin.loaded);
        
        // Unload plugin
        assert!(manager.unload_plugin("test-plugin").is_ok());
        let plugin = manager.get_plugin("test-plugin").unwrap();
        assert!(!plugin.loaded);
        
        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
} 