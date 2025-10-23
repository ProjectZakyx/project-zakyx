// 🔄 Plugin Lifecycle Management für ZAKYX Browser
// Verwaltet den kompletten Lebenszyklus von Plugins: Install, Load, Enable, Update, Uninstall

use std::path::Path;
use crate::plugin::types::{PluginInfo, PluginConfig};
use crate::plugin::discovery::PluginDiscovery;
use crate::plugin::loader::PluginLoader;
use crate::plugin::validator::PluginValidator;
use crate::plugin::management::registry::PluginRegistry;

/// Plugin-Lifecycle-Manager
#[derive(Debug)]
pub struct PluginLifecycleManager {
    registry: PluginRegistry,
    discovery: PluginDiscovery,
    loader: PluginLoader,
    validator: PluginValidator,
    config: PluginConfig,
}

impl PluginLifecycleManager {
    /// Erstellt einen neuen Lifecycle-Manager
    pub fn new(config: PluginConfig) -> Self {
        let registry = PluginRegistry::new(config.clone());
        let discovery = PluginDiscovery::new(config.clone());
        let loader = PluginLoader::new(config.clone());
        let validator = PluginValidator::new(true);
        
        Self {
            registry,
            discovery,
            loader,
            validator,
            config,
        }
    }

    /// Initialisiert den Lifecycle-Manager
    pub fn initialize(&mut self) -> Result<(), String> {
        println!("🔄 Initializing Plugin Lifecycle Manager...");
        
        // Entdecke alle verfügbaren Plugins
        let discovered_plugins = self.discovery.discover_plugins()?;
        
        // Registriere alle entdeckten Plugins
        for (_plugin_id, plugin) in discovered_plugins {
            if let Err(e) = self.registry.register_plugin(plugin) {
                eprintln!("⚠️ Failed to register plugin: {}", e);
            }
        }
        
        // Lade aktivierte Plugins falls konfiguriert
        if self.config.auto_load_enabled {
            self.load_enabled_plugins()?;
        }
        
        println!("✅ Plugin Lifecycle Manager initialized with {} plugins", 
                 self.registry.get_plugin_count());
        Ok(())
    }

    /// Installiert ein neues Plugin
    pub fn install_plugin(&mut self, plugin_path: &Path) -> Result<String, String> {
        println!("📦 Installing plugin from: {}", plugin_path.display());
        
        // Validiere Plugin-Verzeichnis
        if !plugin_path.exists() {
            return Err(format!("Plugin path does not exist: {}", plugin_path.display()));
        }
        
        if !plugin_path.is_dir() {
            return Err("Plugin path must be a directory".to_string());
        }
        
        // Lade Plugin-Informationen
        let plugin_info = self.discovery.load_plugin_from_directory(plugin_path)?;
        let plugin_id = plugin_info.id.clone();
        
        // Prüfe ob Plugin bereits existiert
        if self.registry.contains_plugin(&plugin_id) {
            return Err(format!("Plugin {} already exists", plugin_id));
        }
        
        // Validiere Plugin
        self.validate_plugin(&plugin_info)?;
        
        // Kopiere Plugin in Plugin-Verzeichnis falls nötig
        let target_path = self.config.plugins_directory.join(&plugin_id);
        if plugin_path != target_path {
            self.copy_plugin_files(plugin_path, &target_path)?;
        }
        
        // Aktualisiere Plugin-Pfad
        let mut final_plugin = plugin_info;
        final_plugin.path = target_path;
        
        // Registriere Plugin
        self.registry.register_plugin(final_plugin)?;
        
        // Speichere Plugin-Manifest
        if let Some(plugin) = self.registry.get_plugin(&plugin_id) {
            self.discovery.save_plugin_manifest(plugin)?;
        }
        
        println!("✅ Plugin installed: {}", plugin_id);
        Ok(plugin_id)
    }

    /// Deinstalliert ein Plugin
    pub fn uninstall_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        println!("🗑️ Uninstalling plugin: {}", plugin_id);
        
        // Hole Plugin-Informationen
        let plugin = self.registry.get_plugin(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
        
        let plugin_path = plugin.path.clone();
        let was_loaded = plugin.loaded;
        
        // Entlade Plugin falls geladen
        if was_loaded {
            self.unload_plugin(plugin_id)?;
        }
        
        // Entferne Plugin aus Registry
        self.registry.unregister_plugin(plugin_id)?;
        
        // Entferne Plugin-Verzeichnis
        if plugin_path.exists() {
            std::fs::remove_dir_all(&plugin_path)
                .map_err(|e| format!("Failed to remove plugin directory: {}", e))?;
        }
        
        println!("✅ Plugin uninstalled: {}", plugin_id);
        Ok(())
    }

    /// Aktualisiert ein Plugin
    pub fn update_plugin(&mut self, plugin_id: &str, update_path: Option<&Path>) -> Result<(), String> {
        println!("🔄 Updating plugin: {}", plugin_id);
        
        // Hole Plugin-Informationen
        let (plugin_path, was_loaded, was_enabled) = {
            let current_plugin = self.registry.get_plugin(plugin_id)
                .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
            (current_plugin.path.clone(), current_plugin.loaded, current_plugin.manifest.enabled)
        };
        
        // Lade neues Plugin-Manifest
        let update_source = update_path.unwrap_or(&plugin_path);
        let updated_plugin = self.discovery.load_plugin_from_directory(update_source)?;
        
        // Prüfe ob Update verfügbar (vereinfacht - vergleiche nur Version)
        let current_version = {
            let current_plugin = self.registry.get_plugin(plugin_id)
                .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
            current_plugin.manifest.version.clone()
        };
        
        if updated_plugin.manifest.version == current_version {
            println!("ℹ️ Plugin {} is already up to date", plugin_id);
            return Ok(());
        }
        
        // Validiere Update
        self.validate_plugin(&updated_plugin)?;
        
        // Entlade Plugin falls geladen
        if was_loaded {
            self.unload_plugin(plugin_id)?;
        }
        
        // Kopiere neue Dateien falls Update-Pfad angegeben
        if let Some(update_source) = update_path {
            self.copy_plugin_files(update_source, &plugin_path)?;
        }
        
        // Aktualisiere Plugin in Registry
        let mut final_plugin = updated_plugin;
        final_plugin.path = plugin_path;
        final_plugin.manifest.enabled = was_enabled; // Behalte Enable-Status bei
        
        self.registry.update_plugin(plugin_id, final_plugin)?;
        
        // Speichere aktualisiertes Manifest
        if let Some(plugin) = self.registry.get_plugin(plugin_id) {
            self.discovery.save_plugin_manifest(plugin)?;
        }
        
        // Lade Plugin wieder falls es vorher geladen war
        if was_loaded && was_enabled {
            self.load_plugin(plugin_id)?;
        }
        
        println!("✅ Plugin updated: {}", plugin_id);
        Ok(())
    }

    /// Aktiviert ein Plugin
    pub fn enable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        println!("🟢 Enabling plugin: {}", plugin_id);
        
        // Prüfe Plugin-Existenz und Status
        {
            let plugin = self.registry.get_plugin(plugin_id)
                .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
            
            if plugin.manifest.enabled {
                return Ok(()); // Bereits aktiviert
            }
            
            // Validiere Plugin vor Aktivierung
            self.validate_plugin(plugin)?;
        }
        
        // Hole mutable Plugin-Referenz
        let plugin = self.registry.get_plugin_mut(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
        
        // Aktiviere Plugin
        plugin.manifest.enabled = true;
        
        // Speichere Änderungen
        self.discovery.save_plugin_manifest(plugin)?;
        
        // Lade Plugin falls konfiguriert
        if self.config.auto_load_enabled {
            self.load_plugin(plugin_id)?;
        }
        
        println!("✅ Plugin enabled: {}", plugin_id);
        Ok(())
    }

    /// Deaktiviert ein Plugin
    pub fn disable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        println!("🔴 Disabling plugin: {}", plugin_id);
        
        // Hole Plugin
        let plugin = self.registry.get_plugin_mut(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
        
        if !plugin.manifest.enabled {
            return Ok(()); // Bereits deaktiviert
        }
        
        let was_loaded = plugin.loaded;
        
        // Deaktiviere Plugin
        plugin.manifest.enabled = false;
        
        // Speichere Änderungen
        self.discovery.save_plugin_manifest(plugin)?;
        
        // Entlade Plugin falls geladen
        if was_loaded {
            self.unload_plugin(plugin_id)?;
        }
        
        println!("✅ Plugin disabled: {}", plugin_id);
        Ok(())
    }

    /// Lädt ein Plugin
    pub fn load_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        println!("⬆️ Loading plugin: {}", plugin_id);
        
        // Prüfe ob Plugin existiert und aktiviert ist
        let plugin = self.registry.get_plugin(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
        
        if !plugin.manifest.enabled {
            return Err(format!("Plugin {} is not enabled", plugin_id));
        }
        
        if plugin.loaded {
            return Ok(()); // Bereits geladen
        }
        
        // Validiere Plugin vor dem Laden
        self.validate_plugin(plugin)?;
        
        // Lade Plugin über Loader
        let mut plugins_map = std::collections::HashMap::new();
        plugins_map.insert(plugin_id.to_string(), plugin.clone());
        
        self.loader.load_plugin(plugin_id, &mut plugins_map)?;
        
        // Aktualisiere Registry
        if let Some(updated_plugin) = plugins_map.get(plugin_id) {
            self.registry.update_plugin(plugin_id, updated_plugin.clone())?;
        }
        
        println!("✅ Plugin loaded: {}", plugin_id);
        Ok(())
    }

    /// Entlädt ein Plugin
    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        println!("⬇️ Unloading plugin: {}", plugin_id);
        
        // Prüfe ob Plugin existiert und geladen ist
        let plugin = self.registry.get_plugin(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
        
        if !plugin.loaded {
            return Ok(()); // Bereits entladen
        }
        
        // Entlade Plugin über Loader
        let mut plugins_map = std::collections::HashMap::new();
        plugins_map.insert(plugin_id.to_string(), plugin.clone());
        
        self.loader.unload_plugin(plugin_id, &mut plugins_map)?;
        
        // Aktualisiere Registry
        if let Some(updated_plugin) = plugins_map.get(plugin_id) {
            self.registry.update_plugin(plugin_id, updated_plugin.clone())?;
        }
        
        println!("✅ Plugin unloaded: {}", plugin_id);
        Ok(())
    }

    /// Lädt ein Plugin neu
    pub fn reload_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        println!("🔄 Reloading plugin: {}", plugin_id);
        
        // Plugin-Informationen abrufen (in eigenem Scope)
        let (was_loaded, plugin_path) = {
            let plugin = self.registry.get_plugin(plugin_id)
                .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
            (plugin.loaded, plugin.path.clone())
        };
        
        // Entlade Plugin falls geladen
        if was_loaded {
            self.unload_plugin(plugin_id)?;
        }
        
        // Lade Plugin-Informationen neu
        let reloaded_plugin = self.discovery.load_plugin_from_directory(&plugin_path)?;
        
        // Aktualisiere Registry
        self.registry.update_plugin(plugin_id, reloaded_plugin)?;
        
        // Lade Plugin wieder falls es vorher geladen war
        if was_loaded {
            self.load_plugin(plugin_id)?;
        }
        
        println!("✅ Plugin reloaded: {}", plugin_id);
        Ok(())
    }

    /// Lädt alle aktivierten Plugins
    pub fn load_enabled_plugins(&mut self) -> Result<(), String> {
        println!("⬆️ Loading all enabled plugins...");
        
        let enabled_plugins: Vec<String> = self.registry
            .get_plugins_by_status(Some(true), Some(false))
            .iter()
            .map(|plugin| plugin.id.clone())
            .collect();
        
        let mut loaded_count = 0;
        let mut failed_count = 0;
        
        for plugin_id in enabled_plugins {
            match self.load_plugin(&plugin_id) {
                Ok(_) => loaded_count += 1,
                Err(e) => {
                    eprintln!("⚠️ Failed to load plugin {}: {}", plugin_id, e);
                    failed_count += 1;
                }
            }
        }
        
        println!("✅ Loaded {} plugins, {} failed", loaded_count, failed_count);
        Ok(())
    }

    /// Entlädt alle Plugins
    pub fn unload_all_plugins(&mut self) -> Result<(), String> {
        println!("⬇️ Unloading all plugins...");
        
        let loaded_plugins: Vec<String> = self.registry
            .get_plugins_by_status(None, Some(true))
            .iter()
            .map(|plugin| plugin.id.clone())
            .collect();
        
        let mut unloaded_count = 0;
        let mut failed_count = 0;
        
        for plugin_id in loaded_plugins {
            match self.unload_plugin(&plugin_id) {
                Ok(_) => unloaded_count += 1,
                Err(e) => {
                    eprintln!("⚠️ Failed to unload plugin {}: {}", plugin_id, e);
                    failed_count += 1;
                }
            }
        }
        
        println!("✅ Unloaded {} plugins, {} failed", unloaded_count, failed_count);
        Ok(())
    }

    /// Validiert ein Plugin
    fn validate_plugin(&self, plugin: &PluginInfo) -> Result<(), String> {
        // Validiere Plugin (vereinfacht)
        let validation_result = self.validator.validate_manifest(&plugin.manifest);
        if !validation_result.is_valid() {
            return Err(format!("Plugin validation failed: {:?}", validation_result.errors));
        }
        
        Ok(())
    }

    /// Kopiert Plugin-Dateien
    fn copy_plugin_files(&self, source: &Path, target: &Path) -> Result<(), String> {
        if target.exists() {
            std::fs::remove_dir_all(target)
                .map_err(|e| format!("Failed to remove existing plugin directory: {}", e))?;
        }
        
        std::fs::create_dir_all(target)
            .map_err(|e| format!("Failed to create plugin directory: {}", e))?;
        
        copy_dir_recursive(source, target)
            .map_err(|e| format!("Failed to copy plugin files: {}", e))?;
        
        Ok(())
    }

    /// Bereinigt nicht mehr existierende Plugins
    pub fn cleanup_missing_plugins(&mut self) -> usize {
        println!("🧹 Cleaning up missing plugins...");
        self.registry.cleanup_registry()
    }

    /// Repariert beschädigte Plugins
    pub fn repair_plugins(&mut self) -> Result<Vec<String>, String> {
        println!("🔧 Repairing plugins...");
        
        let mut repaired_plugins = Vec::new();
        let plugin_ids: Vec<String> = self.registry.get_plugin_ids();
        
        for plugin_id in plugin_ids {
            if let Some(plugin) = self.registry.get_plugin(&plugin_id) {
                // Prüfe Plugin-Integrität
                if !plugin.path.exists() {
                    continue; // Wird von cleanup_missing_plugins behandelt
                }
                
                // Versuche Plugin-Manifest zu reparieren
                match self.discovery.load_plugin_from_directory(&plugin.path) {
                    Ok(repaired_plugin) => {
                        if repaired_plugin.manifest != plugin.manifest {
                            self.registry.update_plugin(&plugin_id, repaired_plugin)?;
                            repaired_plugins.push(plugin_id);
                        }
                    }
                    Err(e) => {
                        eprintln!("⚠️ Cannot repair plugin {}: {}", plugin_id, e);
                    }
                }
            }
        }
        
        println!("✅ Repaired {} plugins", repaired_plugins.len());
        Ok(repaired_plugins)
    }

    // Getter für Registry-Zugriff
    pub fn get_registry(&self) -> &PluginRegistry {
        &self.registry
    }

    pub fn get_registry_mut(&mut self) -> &mut PluginRegistry {
        &mut self.registry
    }

    pub fn get_config(&self) -> &PluginConfig {
        &self.config
    }
}

/// Kopiert ein Verzeichnis rekursiv
fn copy_dir_recursive(source: &Path, target: &Path) -> std::io::Result<()> {
    if !target.exists() {
        std::fs::create_dir_all(target)?;
    }
    
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        
        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &target_path)?;
        } else {
            std::fs::copy(&source_path, &target_path)?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    use crate::plugin::types::PluginManifest;

    fn create_test_plugin_dir() -> std::path::PathBuf {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let test_dir = std::env::temp_dir().join(format!("zakyx_test_lifecycle_{}_{}", std::process::id(), nanos));
        if test_dir.exists() {
            let _ = fs::remove_dir_all(&test_dir);
        }
        fs::create_dir_all(&test_dir).unwrap();
        test_dir
    }

    fn create_test_manifest_file(plugin_dir: &Path, name: &str, version: &str, enabled: bool) {
        let manifest = PluginManifest {
            id: Some(name.to_string()),
            name: name.to_string(),
            version: version.to_string(),
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
            minimum_zakyx_version: None,
            dependencies: None,
            settings: None,
            update_url: None,
        };
        
        let manifest_content = serde_json::to_string_pretty(&manifest).unwrap();
        fs::write(plugin_dir.join("plugin.json"), manifest_content).unwrap();
        fs::write(plugin_dir.join("main.js"), "console.log('Test plugin');").unwrap();
    }

    #[test]
    fn test_lifecycle_manager_creation() {
        let test_dir = create_test_plugin_dir();
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            auto_load_enabled: false,
            ..Default::default()
        };
        
        let manager = PluginLifecycleManager::new(config);
        assert_eq!(manager.get_registry().get_plugin_count(), 0);
        
        let _ = fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_plugin_installation() {
        let test_dir = create_test_plugin_dir();
        let plugin_dir = test_dir.join("test-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        create_test_manifest_file(&plugin_dir, "test-plugin", "1.0.0", false);
        
        let plugins_dir = create_test_plugin_dir();
        let config = PluginConfig {
            plugins_directory: plugins_dir.clone(),
            auto_load_enabled: false,
            ..Default::default()
        };
        
        let mut manager = PluginLifecycleManager::new(config);
        
        let plugin_id = manager.install_plugin(&plugin_dir).unwrap();
        assert_eq!(plugin_id, "test-plugin");
        assert_eq!(manager.get_registry().get_plugin_count(), 1);
        assert!(manager.get_registry().contains_plugin("test-plugin"));
        
        // Test duplicate installation
        assert!(manager.install_plugin(&plugin_dir).is_err());
        
        let _ = fs::remove_dir_all(&test_dir);
        let _ = fs::remove_dir_all(&plugins_dir);
    }

    #[test]
    fn test_plugin_enable_disable() {
        let test_dir = create_test_plugin_dir();
        let plugin_dir = test_dir.join("test-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        create_test_manifest_file(&plugin_dir, "test-plugin", "1.0.0", false);
        
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            auto_load_enabled: false,
            ..Default::default()
        };
        
        let mut manager = PluginLifecycleManager::new(config);
        manager.install_plugin(&plugin_dir).unwrap();
        
        // Enable plugin
        assert!(manager.enable_plugin("test-plugin").is_ok());
        let plugin = manager.get_registry().get_plugin("test-plugin").unwrap();
        assert!(plugin.manifest.enabled);
        
        // Disable plugin
        assert!(manager.disable_plugin("test-plugin").is_ok());
        let plugin = manager.get_registry().get_plugin("test-plugin").unwrap();
        assert!(!plugin.manifest.enabled);
        
        let _ = fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_plugin_update() {
        let test_dir = create_test_plugin_dir();
        let plugin_dir = test_dir.join("test-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        create_test_manifest_file(&plugin_dir, "test-plugin", "1.0.0", true);
        
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            auto_load_enabled: false,
            ..Default::default()
        };
        
        let mut manager = PluginLifecycleManager::new(config);
        manager.install_plugin(&plugin_dir).unwrap();
        
        // Update plugin manifest
        create_test_manifest_file(&plugin_dir, "test-plugin", "1.1.0", true);
        
        assert!(manager.update_plugin("test-plugin", None).is_ok());
        let plugin = manager.get_registry().get_plugin("test-plugin").unwrap();
        assert_eq!(plugin.manifest.version, "1.1.0");
        
        let _ = fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_plugin_uninstallation() {
        let test_dir = create_test_plugin_dir();
        let plugin_dir = test_dir.join("test-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        create_test_manifest_file(&plugin_dir, "test-plugin", "1.0.0", false);
        
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            auto_load_enabled: false,
            ..Default::default()
        };
        
        let mut manager = PluginLifecycleManager::new(config);
        manager.install_plugin(&plugin_dir).unwrap();
        
        assert_eq!(manager.get_registry().get_plugin_count(), 1);
        
        assert!(manager.uninstall_plugin("test-plugin").is_ok());
        assert_eq!(manager.get_registry().get_plugin_count(), 0);
        assert!(!manager.get_registry().contains_plugin("test-plugin"));
        
        let _ = fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_cleanup_missing_plugins() {
        let test_dir = create_test_plugin_dir();
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            auto_load_enabled: false,
            ..Default::default()
        };
        
        let mut manager = PluginLifecycleManager::new(config);
        
        // Erstelle Plugin manuell in Registry (ohne Dateien)
        let fake_plugin = crate::plugin::types::PluginInfo {
            id: "fake-plugin".to_string(),
            path: test_dir.join("non-existent"),
            manifest: PluginManifest::default(),
            loaded: false,
            load_error: None,
        };
        
        manager.get_registry_mut().register_plugin(fake_plugin).unwrap();
        assert_eq!(manager.get_registry().get_plugin_count(), 1);
        
        let cleaned_count = manager.cleanup_missing_plugins();
        assert_eq!(cleaned_count, 1);
        assert_eq!(manager.get_registry().get_plugin_count(), 0);
        
        let _ = fs::remove_dir_all(&test_dir);
    }
}