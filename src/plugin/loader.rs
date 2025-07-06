// 🔄 PLUGIN LOADER
// Plugin-Ladelogik und -Verwaltung
// Copyright © 2024 Ora Browser Team

use std::collections::HashMap;
use std::path::Path;
use std::fs;
use crate::plugin::types::{PluginInfo, PluginConfig};
use crate::plugin::validator::PluginValidator;

/// Plugin-Loader-Engine
#[derive(Debug)]
pub struct PluginLoader {
    config: PluginConfig,
    validator: PluginValidator,
    loaded_plugins: HashMap<String, bool>,
}

impl PluginLoader {
    pub fn new(config: PluginConfig) -> Self {
        Self {
            validator: PluginValidator::new(config.validate_permissions),
            config,
            loaded_plugins: HashMap::new(),
        }
    }
    
    /// Lädt alle aktivierten Plugins
    pub fn load_enabled_plugins(&mut self, plugins: &mut HashMap<String, PluginInfo>) -> Result<(), String> {
        let enabled_plugins: Vec<String> = plugins
            .iter()
            .filter(|(_, info)| info.manifest.enabled)
            .map(|(id, _)| id.clone())
            .collect();
        
        println!("🔄 Loading {} enabled plugins...", enabled_plugins.len());
        
        for plugin_id in enabled_plugins {
            if let Err(e) = self.load_plugin(&plugin_id, plugins) {
                println!("❌ Failed to load plugin {}: {}", plugin_id, e);
                if let Some(plugin) = plugins.get_mut(&plugin_id) {
                    plugin.error = Some(e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Lädt ein einzelnes Plugin
    pub fn load_plugin(&mut self, plugin_id: &str, plugins: &mut HashMap<String, PluginInfo>) -> Result<(), String> {
        // Validiere Plugin-Existenz
        let plugin_info = plugins.get(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
        
        if plugin_info.loaded {
            return Ok(());
        }
        
        // Prüfe, ob Plugin bereits geladen ist
        if *self.loaded_plugins.get(plugin_id).unwrap_or(&false) {
            return Ok(());
        }
        
        // Validiere Plugin-Berechtigungen
        if self.config.validate_permissions {
            self.validator.validate_plugin_permissions(&plugin_info.manifest)
                .map_err(|e| e.user_message())?;
        }
        
        // Prüfe Script-Existenz
        let script_path = plugin_info.path.join(&plugin_info.manifest.main_script);
        if !script_path.exists() {
            return Err(format!("Main script not found: {}", plugin_info.manifest.main_script));
        }
        
        // Lade Plugin-Skript
        self.execute_plugin_script(&script_path, plugin_id)?;
        
        // Lade Background-Skripte
        if let Some(background) = &plugin_info.manifest.background {
            for script in &background.scripts {
                let bg_script_path = plugin_info.path.join(script);
                if bg_script_path.exists() {
                    self.execute_plugin_script(&bg_script_path, plugin_id)?;
                }
            }
        }
        
        // Markiere Plugin als geladen
        self.loaded_plugins.insert(plugin_id.to_string(), true);
        
        // Aktualisiere Plugin-Info
        let plugin = plugins.get_mut(plugin_id).unwrap();
        plugin.loaded = true;
        plugin.error = None;
        
        println!("✅ Plugin loaded: {} v{}", plugin.manifest.name, plugin.manifest.version);
        Ok(())
    }
    
    /// Entlädt ein Plugin
    pub fn unload_plugin(&mut self, plugin_id: &str, plugins: &mut HashMap<String, PluginInfo>) -> Result<(), String> {
        let plugin = plugins.get_mut(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
        
        if !plugin.loaded {
            return Ok(());
        }
        
        println!("🔄 Unloading plugin: {}", plugin_id);
        
        // Führe Plugin-Cleanup durch
        self.cleanup_plugin(plugin_id, plugin)?;
        
        // Markiere Plugin als entladen
        self.loaded_plugins.insert(plugin_id.to_string(), false);
        plugin.loaded = false;
        
        println!("✅ Plugin unloaded: {}", plugin.manifest.name);
        Ok(())
    }
    
    /// Entlädt alle Plugins
    pub fn unload_all_plugins(&mut self, plugins: &mut HashMap<String, PluginInfo>) -> Result<(), String> {
        let loaded_plugins: Vec<String> = plugins
            .iter()
            .filter(|(_, info)| info.loaded)
            .map(|(id, _)| id.clone())
            .collect();
        
        println!("🔄 Unloading {} plugins...", loaded_plugins.len());
        
        for plugin_id in loaded_plugins {
            if let Err(e) = self.unload_plugin(&plugin_id, plugins) {
                println!("❌ Failed to unload plugin {}: {}", plugin_id, e);
            }
        }
        
        Ok(())
    }
    
    /// Führt ein Plugin-Skript aus
    fn execute_plugin_script(&self, script_path: &Path, plugin_id: &str) -> Result<(), String> {
        // Für jetzt nur simulierte Ausführung
        // In einer echten Implementierung würde hier das JavaScript ausgeführt
        let script_content = fs::read_to_string(script_path)
            .map_err(|e| format!("Failed to read script {}: {}", script_path.display(), e))?;
        
        // Grundlegende Syntaxprüfung
        if script_content.trim().is_empty() {
            return Err("Script file is empty".to_string());
        }
        
        // Simuliere Script-Ausführung
        println!("🔄 Executing script: {} for plugin: {}", script_path.display(), plugin_id);
        
        // Hier würde normalerweise ein JavaScript-Engine das Script ausführen
        // z.B. V8, QuickJS, oder ein anderer JavaScript-Interpreter
        
        Ok(())
    }
    
    /// Führt Plugin-Cleanup durch
    fn cleanup_plugin(&self, plugin_id: &str, plugin: &PluginInfo) -> Result<(), String> {
        // Cleanup-Logik für verschiedene Plugin-Typen
        
        // Background-Skripte beenden
        if let Some(background) = &plugin.manifest.background {
            if background.persistent.unwrap_or(false) {
                println!("🧹 Stopping persistent background scripts for: {}", plugin_id);
            }
        }
        
        // Content-Skripte entfernen
        if let Some(content_scripts) = &plugin.manifest.content_scripts {
            println!("🧹 Removing {} content scripts for: {}", content_scripts.len(), plugin_id);
        }
        
        // Browser-Actions entfernen
        if plugin.manifest.browser_action.is_some() {
            println!("🧹 Removing browser action for: {}", plugin_id);
        }
        
        // Plugin-spezifische Cleanup-Logik
        self.run_plugin_cleanup_hooks(plugin_id)?;
        
        Ok(())
    }
    
    /// Führt Plugin-spezifische Cleanup-Hooks aus
    fn run_plugin_cleanup_hooks(&self, plugin_id: &str) -> Result<(), String> {
        // Hier könnten Plugin-spezifische Cleanup-Funktionen aufgerufen werden
        println!("🧹 Running cleanup hooks for: {}", plugin_id);
        
        // Beispiel: Entferne Event-Listener, beende Timer, etc.
        
        Ok(())
    }
    
    /// Neu lädt ein Plugin
    pub fn reload_plugin(&mut self, plugin_id: &str, plugins: &mut HashMap<String, PluginInfo>) -> Result<(), String> {
        println!("🔄 Reloading plugin: {}", plugin_id);
        
        // Erst entladen
        self.unload_plugin(plugin_id, plugins)?;
        
        // Dann neu laden
        self.load_plugin(plugin_id, plugins)?;
        
        println!("✅ Plugin reloaded: {}", plugin_id);
        Ok(())
    }
    
    /// Prüft, ob ein Plugin geladen ist
    pub fn is_plugin_loaded(&self, plugin_id: &str) -> bool {
        self.loaded_plugins.get(plugin_id).unwrap_or(&false).clone()
    }
    
    /// Gibt alle geladenen Plugin-IDs zurück
    pub fn get_loaded_plugin_ids(&self) -> Vec<String> {
        self.loaded_plugins
            .iter()
            .filter(|(_, &loaded)| loaded)
            .map(|(id, _)| id.clone())
            .collect()
    }
    
    /// Gibt Statistiken über geladene Plugins zurück
    pub fn get_loader_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        let loaded_count = self.loaded_plugins.values().filter(|&&loaded| loaded).count();
        let total_count = self.loaded_plugins.len();
        
        stats.insert("loaded_plugins".to_string(), loaded_count);
        stats.insert("total_plugins".to_string(), total_count);
        stats.insert("unloaded_plugins".to_string(), total_count - loaded_count);
        
        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::types::PluginManifest;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    
    fn create_test_plugin_dir() -> std::path::PathBuf {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let test_dir = std::env::temp_dir().join(format!("ora_test_loader_{}_{}", std::process::id(), nanos));
        if test_dir.exists() {
            let _ = fs::remove_dir_all(&test_dir);
        }
        fs::create_dir_all(&test_dir).unwrap();
        test_dir
    }
    
    fn create_test_plugin_info(plugin_id: &str, enabled: bool) -> PluginInfo {
        let test_dir = create_test_plugin_dir();
        let plugin_dir = test_dir.join(plugin_id);
        fs::create_dir_all(&plugin_dir).unwrap();
        
        // Erstelle test script
        fs::write(plugin_dir.join("main.js"), "console.log('Test plugin loaded');").unwrap();
        
        PluginInfo {
            id: plugin_id.to_string(),
            manifest: PluginManifest {
                id: Some(plugin_id.to_string()),
                name: format!("Test Plugin {}", plugin_id),
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
            },
            path: plugin_dir,
            loaded: false,
            error: None,
        }
    }
    
    #[test]
    fn test_plugin_loader_creation() {
        let config = PluginConfig::default();
        let loader = PluginLoader::new(config);
        
        assert!(loader.loaded_plugins.is_empty());
    }
    
    #[test]
    fn test_load_single_plugin() {
        let config = PluginConfig::default();
        let mut loader = PluginLoader::new(config);
        
        let plugin_info = create_test_plugin_info("test-plugin", true);
        let mut plugins = HashMap::new();
        plugins.insert("test-plugin".to_string(), plugin_info);
        
        assert!(loader.load_plugin("test-plugin", &mut plugins).is_ok());
        assert!(loader.is_plugin_loaded("test-plugin"));
        
        let plugin = plugins.get("test-plugin").unwrap();
        assert!(plugin.loaded);
        assert!(plugin.error.is_none());
    }
    
    #[test]
    fn test_unload_plugin() {
        let config = PluginConfig::default();
        let mut loader = PluginLoader::new(config);
        
        let plugin_info = create_test_plugin_info("test-plugin", true);
        let mut plugins = HashMap::new();
        plugins.insert("test-plugin".to_string(), plugin_info);
        
        // Erst laden
        loader.load_plugin("test-plugin", &mut plugins).unwrap();
        assert!(loader.is_plugin_loaded("test-plugin"));
        
        // Dann entladen
        loader.unload_plugin("test-plugin", &mut plugins).unwrap();
        assert!(!loader.is_plugin_loaded("test-plugin"));
        
        let plugin = plugins.get("test-plugin").unwrap();
        assert!(!plugin.loaded);
    }
    
    #[test]
    fn test_load_enabled_plugins() {
        let config = PluginConfig::default();
        let mut loader = PluginLoader::new(config);
        
        let plugin1 = create_test_plugin_info("plugin1", true);
        let plugin2 = create_test_plugin_info("plugin2", false);
        let plugin3 = create_test_plugin_info("plugin3", true);
        
        let mut plugins = HashMap::new();
        plugins.insert("plugin1".to_string(), plugin1);
        plugins.insert("plugin2".to_string(), plugin2);
        plugins.insert("plugin3".to_string(), plugin3);
        
        loader.load_enabled_plugins(&mut plugins).unwrap();
        
        assert!(loader.is_plugin_loaded("plugin1"));
        assert!(!loader.is_plugin_loaded("plugin2"));
        assert!(loader.is_plugin_loaded("plugin3"));
    }
    
    #[test]
    fn test_reload_plugin() {
        let config = PluginConfig::default();
        let mut loader = PluginLoader::new(config);
        
        let plugin_info = create_test_plugin_info("test-plugin", true);
        let mut plugins = HashMap::new();
        plugins.insert("test-plugin".to_string(), plugin_info);
        
        // Erst laden
        loader.load_plugin("test-plugin", &mut plugins).unwrap();
        assert!(loader.is_plugin_loaded("test-plugin"));
        
        // Neu laden
        loader.reload_plugin("test-plugin", &mut plugins).unwrap();
        assert!(loader.is_plugin_loaded("test-plugin"));
    }
    
    #[test]
    fn test_loader_stats() {
        let config = PluginConfig::default();
        let mut loader = PluginLoader::new(config);
        
        let plugin1 = create_test_plugin_info("plugin1", true);
        let plugin2 = create_test_plugin_info("plugin2", true);
        
        let mut plugins = HashMap::new();
        plugins.insert("plugin1".to_string(), plugin1);
        plugins.insert("plugin2".to_string(), plugin2);
        
        loader.load_plugin("plugin1", &mut plugins).unwrap();
        loader.load_plugin("plugin2", &mut plugins).unwrap();
        
        let stats = loader.get_loader_stats();
        assert_eq!(stats.get("loaded_plugins"), Some(&2));
        assert_eq!(stats.get("total_plugins"), Some(&2));
        assert_eq!(stats.get("unloaded_plugins"), Some(&0));
    }
} 