// 🔌 Plugin Management Module
// Modulare Plugin-Verwaltung für ZAKYX Browser

pub mod registry;
pub mod lifecycle;
pub mod operations;

// Re-exports für einfachen Zugriff
pub use registry::PluginRegistry;
pub use lifecycle::PluginLifecycleManager;
pub use operations::{
    PluginOperationsManager, 
    DetailedPluginInfo, 
    ExtendedPluginStats, 
    ExportFormat, 
    DependencyIssue, 
    DependencyIssueType
};

// Hauptklasse für Plugin-Management
use crate::plugin::types::{PluginInfo, PluginConfig, PluginStats};
use crate::plugin::discovery::PluginDiscovery;
use crate::plugin::loader::PluginLoader;
use crate::plugin::validator::PluginValidator;
use std::path::Path;

/// Unified Plugin Manager - Hauptschnittstelle für alle Plugin-Operationen
#[derive(Debug)]
pub struct PluginManager {
    lifecycle: PluginLifecycleManager,
    operations: PluginOperationsManager,
    config: PluginConfig,
}

impl PluginManager {
    /// Erstellt einen neuen Plugin-Manager
    pub fn new() -> Self {
        let config = PluginConfig::default();
        Self::with_config(config)
    }
    
    /// Erstellt einen neuen Plugin-Manager mit benutzerdefinierter Konfiguration
    pub fn with_config(config: PluginConfig) -> Self {
        let lifecycle = PluginLifecycleManager::new(config.clone());
        let registry = PluginRegistry::new(config.clone());
        let operations = PluginOperationsManager::new(registry);
        
        Self {
            lifecycle,
            operations,
            config,
        }
    }
    
    /// Initialisiert den Plugin-Manager
    pub fn initialize(&mut self) -> Result<(), String> {
        println!("🔌 Initializing Unified Plugin Manager...");
        
        // Initialisiere Lifecycle-Manager
        self.lifecycle.initialize()?;
        
        // Synchronisiere Operations-Manager mit Registry
        self.sync_operations_with_lifecycle();
        
        let plugin_count = self.lifecycle.get_registry().get_plugin_count();
        println!("✅ Plugin Manager initialized with {} plugins", plugin_count);
        Ok(())
    }

    // === LIFECYCLE OPERATIONS ===
    
    /// Installiert ein neues Plugin
    pub fn install_plugin(&mut self, plugin_path: &Path) -> Result<String, String> {
        let result = self.lifecycle.install_plugin(plugin_path);
        self.sync_operations_with_lifecycle();
        result
    }
    
    /// Deinstalliert ein Plugin
    pub fn uninstall_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let result = self.lifecycle.uninstall_plugin(plugin_id);
        self.sync_operations_with_lifecycle();
        result
    }
    
    /// Aktualisiert ein Plugin
    pub fn update_plugin(&mut self, plugin_id: &str, update_path: Option<&Path>) -> Result<(), String> {
        let result = self.lifecycle.update_plugin(plugin_id, update_path);
        self.sync_operations_with_lifecycle();
        result
    }
    
    /// Aktiviert ein Plugin
    pub fn enable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let result = self.lifecycle.enable_plugin(plugin_id);
        self.sync_operations_with_lifecycle();
        result
    }
    
    /// Deaktiviert ein Plugin
    pub fn disable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let result = self.lifecycle.disable_plugin(plugin_id);
        self.sync_operations_with_lifecycle();
        result
    }
    
    /// Lädt ein Plugin
    pub fn load_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let result = self.lifecycle.load_plugin(plugin_id);
        self.sync_operations_with_lifecycle();
        result
    }
    
    /// Entlädt ein Plugin
    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let result = self.lifecycle.unload_plugin(plugin_id);
        self.sync_operations_with_lifecycle();
        result
    }
    
    /// Lädt ein Plugin neu
    pub fn reload_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let result = self.lifecycle.reload_plugin(plugin_id);
        self.sync_operations_with_lifecycle();
        result
    }
    
    /// Lädt alle aktivierten Plugins
    pub fn load_enabled_plugins(&mut self) -> Result<(), String> {
        let result = self.lifecycle.load_enabled_plugins();
        self.sync_operations_with_lifecycle();
        result
    }
    
    /// Entlädt alle Plugins
    pub fn unload_all_plugins(&mut self) -> Result<(), String> {
        let result = self.lifecycle.unload_all_plugins();
        self.sync_operations_with_lifecycle();
        result
    }

    // === QUERY OPERATIONS ===
    
    /// Hole alle Plugins
    pub fn list_all_plugins(&self) -> Vec<&PluginInfo> {
        self.operations.list_all_plugins()
    }
    
    /// Hole aktivierte Plugins
    pub fn list_enabled_plugins(&self) -> Vec<&PluginInfo> {
        self.operations.list_enabled_plugins()
    }
    
    /// Hole deaktivierte Plugins
    pub fn list_disabled_plugins(&self) -> Vec<&PluginInfo> {
        self.operations.list_disabled_plugins()
    }
    
    /// Hole geladene Plugins
    pub fn list_loaded_plugins(&self) -> Vec<&PluginInfo> {
        self.operations.list_loaded_plugins()
    }
    
    /// Hole nicht geladene Plugins
    pub fn list_unloaded_plugins(&self) -> Vec<&PluginInfo> {
        self.operations.list_unloaded_plugins()
    }
    
    /// Hole fehlgeschlagene Plugins
    pub fn list_failed_plugins(&self) -> Vec<&PluginInfo> {
        self.operations.list_failed_plugins()
    }
    
    /// Suche Plugins
    pub fn search_plugins(&self, query: &str) -> Vec<&PluginInfo> {
        self.operations.search_plugins(query)
    }
    
    /// Suche Plugins nach Autor
    pub fn search_plugins_by_author(&self, author: &str) -> Vec<&PluginInfo> {
        self.operations.search_plugins_by_author(author)
    }
    
    /// Suche Plugins nach Version
    pub fn search_plugins_by_version(&self, version: &str) -> Vec<&PluginInfo> {
        self.operations.search_plugins_by_version(version)
    }
    
    /// Hole Plugins mit bestimmter Berechtigung
    pub fn get_plugins_with_permission(&self, permission: &str) -> Vec<&PluginInfo> {
        self.operations.get_plugins_with_permission(permission)
    }
    
    /// Hole Plugin-Informationen
    pub fn get_plugin(&self, plugin_id: &str) -> Option<&PluginInfo> {
        self.operations.get_plugin_info(plugin_id)
    }
    
    /// Hole detaillierte Plugin-Informationen
    pub fn get_detailed_plugin_info(&self, plugin_id: &str) -> Option<DetailedPluginInfo> {
        self.operations.get_detailed_plugin_info(plugin_id)
    }

    // === STATISTICS & REPORTING ===
    
    /// Hole Plugin-Statistiken
    pub fn get_plugin_stats(&mut self) -> &PluginStats {
        self.sync_operations_with_lifecycle();
        self.operations.get_plugin_stats()
    }
    
    /// Hole erweiterte Statistiken
    pub fn get_extended_stats(&mut self) -> ExtendedPluginStats {
        self.sync_operations_with_lifecycle();
        self.operations.get_extended_stats()
    }
    
    /// Generiere Plugin-Report
    pub fn generate_plugin_report(&mut self) -> String {
        self.sync_operations_with_lifecycle();
        self.operations.generate_plugin_report()
    }
    
    /// Exportiere Plugin-Liste
    pub fn export_plugin_list(&self, format: ExportFormat) -> Result<String, String> {
        self.operations.export_plugin_list(format)
    }

    // === MAINTENANCE OPERATIONS ===
    
    /// Bereinige nicht mehr existierende Plugins
    pub fn cleanup_missing_plugins(&mut self) -> usize {
        let result = self.lifecycle.cleanup_missing_plugins();
        self.sync_operations_with_lifecycle();
        result
    }
    
    /// Repariere beschädigte Plugins
    pub fn repair_plugins(&mut self) -> Result<Vec<String>, String> {
        let result = self.lifecycle.repair_plugins();
        self.sync_operations_with_lifecycle();
        result
    }
    
    /// Validiere Plugin-Abhängigkeiten
    pub fn validate_dependencies(&self) -> Vec<DependencyIssue> {
        self.operations.validate_dependencies()
    }
    
    /// Validiere Plugin-Registry
    pub fn validate_registry(&self) -> Vec<String> {
        self.lifecycle.get_registry().validate_registry()
    }

    // === IMPORT/EXPORT ===
    
    /// Exportiere Registry
    pub fn export_registry(&self) -> Result<String, serde_json::Error> {
        self.lifecycle.get_registry().export_registry()
    }
    
    /// Importiere Registry
    pub fn import_registry(&mut self, json_data: &str) -> Result<usize, String> {
        let result = self.lifecycle.get_registry_mut().import_registry(json_data);
        self.sync_operations_with_lifecycle();
        result
    }

    // === UTILITY METHODS ===
    
    /// Synchronisiere Operations-Manager mit Lifecycle-Registry
    fn sync_operations_with_lifecycle(&mut self) {
        // Da wir keine direkte Möglichkeit haben, die Registry zu klonen,
        // erstellen wir einen neuen Operations-Manager mit der aktuellen Registry
        // Dies ist ein vereinfachter Ansatz - in einer echten Implementation
        // würde man hier eine effizientere Synchronisation implementieren
        let mut registry_clone = PluginRegistry::new(self.config.clone());
        
        // Kopiere alle Plugins aus der Lifecycle-Registry
        for (_plugin_id, plugin) in self.lifecycle.get_registry().get_all_plugins() {
            let _ = registry_clone.register_plugin(plugin.clone());
        }
        
        self.operations = PluginOperationsManager::new(registry_clone);
    }
    
    /// Hole Registry-Zusammenfassung
    pub fn get_registry_summary(&mut self) -> String {
        self.lifecycle.get_registry_mut().get_registry_summary()
    }
    
    /// Prüfe ob Plugin existiert
    pub fn plugin_exists(&self, plugin_id: &str) -> bool {
        self.lifecycle.get_registry().contains_plugin(plugin_id)
    }
    
    /// Hole Plugin-Anzahl
    pub fn get_plugin_count(&self) -> usize {
        self.lifecycle.get_registry().get_plugin_count()
    }
    
    /// Prüfe ob Registry leer ist
    pub fn is_empty(&self) -> bool {
        self.lifecycle.get_registry().is_empty()
    }

    // === CONFIGURATION ===
    
    /// Hole Konfiguration
    pub fn get_config(&self) -> &PluginConfig {
        &self.config
    }
    
    /// Aktualisiere Konfiguration
    pub fn update_config(&mut self, new_config: PluginConfig) -> Result<(), String> {
        self.config = new_config.clone();
        
        // Erstelle neue Manager mit neuer Konfiguration
        self.lifecycle = PluginLifecycleManager::new(new_config.clone());
        let registry = PluginRegistry::new(new_config);
        self.operations = PluginOperationsManager::new(registry);
        
        // Re-initialisiere
        self.initialize()
    }

    // === ADVANCED OPERATIONS ===
    
    /// Führe Batch-Operation auf mehreren Plugins aus
    pub fn batch_operation<F>(&mut self, plugin_ids: &[String], operation: F) -> Vec<Result<(), String>>
    where
        F: Fn(&mut Self, &str) -> Result<(), String>,
    {
        let mut results = Vec::new();
        
        for plugin_id in plugin_ids {
            let result = operation(self, plugin_id);
            results.push(result);
        }
        
        self.sync_operations_with_lifecycle();
        results
    }
    
    /// Aktiviere alle Plugins eines Autors
    pub fn enable_plugins_by_author(&mut self, author: &str) -> Vec<Result<(), String>> {
        let plugin_ids: Vec<String> = self.search_plugins_by_author(author)
            .iter()
            .map(|plugin| plugin.id.clone())
            .collect();
        
        self.batch_operation(&plugin_ids, |manager, id| manager.enable_plugin(id))
    }
    
    /// Deaktiviere alle Plugins mit bestimmter Berechtigung
    pub fn disable_plugins_with_permission(&mut self, permission: &str) -> Vec<Result<(), String>> {
        let plugin_ids: Vec<String> = self.get_plugins_with_permission(permission)
            .iter()
            .map(|plugin| plugin.id.clone())
            .collect();
        
        self.batch_operation(&plugin_ids, |manager, id| manager.disable_plugin(id))
    }
    
    /// Aktualisiere alle Plugins
    pub fn update_all_plugins(&mut self) -> Vec<Result<(), String>> {
        let plugin_ids: Vec<String> = self.list_all_plugins()
            .iter()
            .map(|plugin| plugin.id.clone())
            .collect();
        
        self.batch_operation(&plugin_ids, |manager, id| manager.update_plugin(id, None))
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    use crate::plugin::types::PluginManifest;

    fn create_test_plugin_dir() -> std::path::PathBuf {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let test_dir = std::env::temp_dir().join(format!("zakyx_test_unified_{}_{}", std::process::id(), nanos));
        if test_dir.exists() {
            let _ = fs::remove_dir_all(&test_dir);
        }
        fs::create_dir_all(&test_dir).unwrap();
        test_dir
    }

    fn create_test_manifest_file(plugin_dir: &Path, name: &str, enabled: bool) {
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
    fn test_unified_manager_creation() {
        let test_dir = create_test_plugin_dir();
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            auto_load_enabled: false,
            ..Default::default()
        };
        
        let manager = PluginManager::with_config(config);
        assert_eq!(manager.get_plugin_count(), 0);
        assert!(manager.is_empty());
        
        let _ = fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_unified_manager_initialization() {
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
        assert_eq!(manager.get_plugin_count(), 1);
        assert!(manager.plugin_exists("test-plugin"));
        
        let _ = fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_unified_lifecycle_operations() {
        let test_dir = create_test_plugin_dir();
        let plugin_dir = test_dir.join("test-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        create_test_manifest_file(&plugin_dir, "test-plugin", false);
        
        let plugins_dir = create_test_plugin_dir();
        let config = PluginConfig {
            plugins_directory: plugins_dir.clone(),
            auto_load_enabled: false,
            ..Default::default()
        };
        
        let mut manager = PluginManager::with_config(config);
        
        // Install
        let plugin_id = manager.install_plugin(&plugin_dir).unwrap();
        assert_eq!(plugin_id, "test-plugin");
        assert_eq!(manager.get_plugin_count(), 1);
        
        // Enable
        assert!(manager.enable_plugin("test-plugin").is_ok());
        let enabled_plugins = manager.list_enabled_plugins();
        assert_eq!(enabled_plugins.len(), 1);
        
        // Disable
        assert!(manager.disable_plugin("test-plugin").is_ok());
        let disabled_plugins = manager.list_disabled_plugins();
        assert_eq!(disabled_plugins.len(), 1);
        
        // Uninstall
        assert!(manager.uninstall_plugin("test-plugin").is_ok());
        assert_eq!(manager.get_plugin_count(), 0);
        
        let _ = fs::remove_dir_all(&test_dir);
        let _ = fs::remove_dir_all(&plugins_dir);
    }

    #[test]
    fn test_unified_query_operations() {
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
        
        // Test queries
        assert_eq!(manager.list_all_plugins().len(), 2);
        assert_eq!(manager.list_enabled_plugins().len(), 1);
        assert_eq!(manager.list_disabled_plugins().len(), 1);
        
        // Test search
        let search_results = manager.search_plugins("plugin1");
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].id, "plugin1");
        
        // Test detailed info
        let detailed = manager.get_detailed_plugin_info("plugin1");
        assert!(detailed.is_some());
        
        let _ = fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_unified_batch_operations() {
        let test_dir = create_test_plugin_dir();
        let plugin_dir1 = test_dir.join("plugin1");
        let plugin_dir2 = test_dir.join("plugin2");
        fs::create_dir_all(&plugin_dir1).unwrap();
        fs::create_dir_all(&plugin_dir2).unwrap();
        create_test_manifest_file(&plugin_dir1, "plugin1", false);
        create_test_manifest_file(&plugin_dir2, "plugin2", false);
        
        let config = PluginConfig {
            plugins_directory: test_dir.clone(),
            auto_load_enabled: false,
            ..Default::default()
        };
        
        let mut manager = PluginManager::with_config(config);
        manager.initialize().unwrap();
        
        // Batch enable
        let plugin_ids = vec!["plugin1".to_string(), "plugin2".to_string()];
        let results = manager.batch_operation(&plugin_ids, |mgr, id| mgr.enable_plugin(id));
        
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.is_ok()));
        assert_eq!(manager.list_enabled_plugins().len(), 2);
        
        let _ = fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_unified_stats_and_reporting() {
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
        
        // Test stats
        let stats = manager.get_plugin_stats();
        assert_eq!(stats.total_plugins, 1);
        assert_eq!(stats.enabled_plugins, 1);
        
        // Test extended stats
        let extended_stats = manager.get_extended_stats();
        assert_eq!(extended_stats.basic.total_plugins, 1);
        
        // Test report generation
        let report = manager.generate_plugin_report();
        assert!(report.contains("PLUGIN REPORT"));
        assert!(report.contains("test-plugin"));
        
        let _ = fs::remove_dir_all(&test_dir);
    }
}