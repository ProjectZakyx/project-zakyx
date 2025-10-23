// 📋 Plugin Registry für ZAKYX Browser
// Zentrale Verwaltung aller Plugin-Instanzen und deren Metadaten

use std::collections::HashMap;
use crate::plugin::types::{PluginInfo, PluginConfig, PluginStats};
use crate::plugin::discovery::PluginDiscovery;
use crate::plugin::loader::PluginLoader;

/// Zentrale Registry für alle Plugin-Instanzen
#[derive(Debug)]
pub struct PluginRegistry {
    plugins: HashMap<String, PluginInfo>,
    config: PluginConfig,
    stats_cache: Option<PluginStats>,
}

impl PluginRegistry {
    /// Erstellt eine neue Plugin-Registry
    pub fn new(config: PluginConfig) -> Self {
        Self {
            plugins: HashMap::new(),
            config,
            stats_cache: None,
        }
    }

    /// Registriert ein neues Plugin
    pub fn register_plugin(&mut self, plugin: PluginInfo) -> Result<(), String> {
        let plugin_id = plugin.id.clone();
        
        if self.plugins.contains_key(&plugin_id) {
            return Err(format!("Plugin {} already registered", plugin_id));
        }
        
        self.plugins.insert(plugin_id.clone(), plugin);
        self.invalidate_stats_cache();
        
        println!("📋 Plugin registered: {}", plugin_id);
        Ok(())
    }

    /// Entfernt ein Plugin aus der Registry
    pub fn unregister_plugin(&mut self, plugin_id: &str) -> Result<PluginInfo, String> {
        let plugin = self.plugins.remove(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;
        
        self.invalidate_stats_cache();
        println!("📋 Plugin unregistered: {}", plugin_id);
        Ok(plugin)
    }

    /// Aktualisiert Plugin-Informationen
    pub fn update_plugin(&mut self, plugin_id: &str, updated_plugin: PluginInfo) -> Result<(), String> {
        if !self.plugins.contains_key(plugin_id) {
            return Err(format!("Plugin {} not found", plugin_id));
        }
        
        self.plugins.insert(plugin_id.to_string(), updated_plugin);
        self.invalidate_stats_cache();
        
        println!("📋 Plugin updated: {}", plugin_id);
        Ok(())
    }

    /// Prüft ob Plugin existiert
    pub fn contains_plugin(&self, plugin_id: &str) -> bool {
        self.plugins.contains_key(plugin_id)
    }

    /// Hole Plugin-Informationen
    pub fn get_plugin(&self, plugin_id: &str) -> Option<&PluginInfo> {
        self.plugins.get(plugin_id)
    }

    /// Hole mutable Plugin-Referenz
    pub fn get_plugin_mut(&mut self, plugin_id: &str) -> Option<&mut PluginInfo> {
        if self.plugins.contains_key(plugin_id) {
            self.invalidate_stats_cache();
        }
        self.plugins.get_mut(plugin_id)
    }

    /// Hole alle Plugins
    pub fn get_all_plugins(&self) -> &HashMap<String, PluginInfo> {
        &self.plugins
    }

    /// Hole alle Plugin-IDs
    pub fn get_plugin_ids(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    /// Filtere Plugins nach Status
    pub fn get_plugins_by_status(&self, enabled: Option<bool>, loaded: Option<bool>) -> Vec<&PluginInfo> {
        self.plugins.values()
            .filter(|plugin| {
                if let Some(enabled_filter) = enabled {
                    if plugin.manifest.enabled != enabled_filter {
                        return false;
                    }
                }
                
                if let Some(loaded_filter) = loaded {
                    if plugin.loaded != loaded_filter {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }

    /// Suche Plugins nach Name oder Beschreibung
    pub fn search_plugins(&self, query: &str) -> Vec<&PluginInfo> {
        let query_lower = query.to_lowercase();
        
        self.plugins.values()
            .filter(|plugin| {
                plugin.manifest.name.to_lowercase().contains(&query_lower) ||
                plugin.manifest.description.to_lowercase().contains(&query_lower) ||
                plugin.manifest.author.to_lowercase().contains(&query_lower) ||
                plugin.id.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Hole Plugins nach Berechtigung
    pub fn get_plugins_with_permission(&self, permission: &str) -> Vec<&PluginInfo> {
        self.plugins.values()
            .filter(|plugin| plugin.manifest.permissions.contains(&permission.to_string()))
            .collect()
    }

    /// Berechne Plugin-Statistiken
    pub fn calculate_stats(&mut self) -> &PluginStats {
        if self.stats_cache.is_none() {
            let mut stats = PluginStats {
                total_plugins: self.plugins.len(),
                enabled_plugins: 0,
                loaded_plugins: 0,
                failed_plugins: 0,
                permissions_count: HashMap::new(),
                version_distribution: HashMap::new(),
                author_distribution: HashMap::new(),
                total_size: 0,
            };

            for plugin in self.plugins.values() {
                // Status-Zählung
                if plugin.manifest.enabled {
                    stats.enabled_plugins += 1;
                }
                
                if plugin.loaded {
                    stats.loaded_plugins += 1;
                }
                
                if plugin.load_error.is_some() {
                    stats.failed_plugins += 1;
                }

                // Berechtigungen zählen
                for permission in &plugin.manifest.permissions {
                    *stats.permissions_count.entry(permission.clone()).or_insert(0) += 1;
                }

                // Versionsverteilung
                *stats.version_distribution.entry(plugin.manifest.version.clone()).or_insert(0) += 1;

                // Autor-Verteilung
                *stats.author_distribution.entry(plugin.manifest.author.clone()).or_insert(0) += 1;

                // Größe berechnen (vereinfacht)
                if let Ok(metadata) = std::fs::metadata(&plugin.path) {
                    stats.total_size += metadata.len();
                }
            }

            self.stats_cache = Some(stats);
        }

        self.stats_cache.as_ref().unwrap()
    }

    /// Hole gecachte Statistiken oder berechne neue
    pub fn get_stats(&mut self) -> &PluginStats {
        self.calculate_stats()
    }

    /// Invalidiere Statistik-Cache
    fn invalidate_stats_cache(&mut self) {
        self.stats_cache = None;
    }

    /// Exportiere Registry als JSON
    pub fn export_registry(&self) -> Result<String, serde_json::Error> {
        let export_data = serde_json::json!({
            "plugins": self.plugins,
            "config": self.config,
            "export_timestamp": chrono::Utc::now().to_rfc3339(),
            "version": "1.0"
        });

        serde_json::to_string_pretty(&export_data)
    }

    /// Importiere Registry aus JSON
    pub fn import_registry(&mut self, json_data: &str) -> Result<usize, String> {
        let import_data: serde_json::Value = serde_json::from_str(json_data)
            .map_err(|e| format!("Invalid JSON: {}", e))?;
        
        if let Some(plugins_obj) = import_data.get("plugins") {
            let imported_plugins: HashMap<String, PluginInfo> = serde_json::from_value(plugins_obj.clone())
                .map_err(|e| format!("Invalid plugin data: {}", e))?;
            
            let mut imported_count = 0;
            for (plugin_id, plugin_info) in imported_plugins {
                if !self.plugins.contains_key(&plugin_id) {
                    self.plugins.insert(plugin_id, plugin_info);
                    imported_count += 1;
                }
            }

            self.invalidate_stats_cache();
            println!("📥 Imported {} plugins", imported_count);
            Ok(imported_count)
        } else {
            Err("No plugins found in import data".to_string())
        }
    }

    /// Bereinige Registry (entferne nicht existierende Plugins)
    pub fn cleanup_registry(&mut self) -> usize {
        let initial_count = self.plugins.len();
        
        self.plugins.retain(|_id, plugin| {
            plugin.path.exists()
        });
        
        let removed_count = initial_count - self.plugins.len();
        if removed_count > 0 {
            self.invalidate_stats_cache();
            println!("🧹 Cleaned up {} non-existent plugins", removed_count);
        }
        
        removed_count
    }

    /// Validiere Registry-Integrität
    pub fn validate_registry(&self) -> Vec<String> {
        let mut issues = Vec::new();
        
        for (plugin_id, plugin) in &self.plugins {
            // Prüfe Plugin-ID Konsistenz
            if plugin.id != *plugin_id {
                issues.push(format!("Plugin ID mismatch: {} vs {}", plugin_id, plugin.id));
            }
            
            // Prüfe Pfad-Existenz
            if !plugin.path.exists() {
                issues.push(format!("Plugin path does not exist: {} ({})", plugin_id, plugin.path.display()));
            }
            
            // Prüfe Manifest-Konsistenz
            if plugin.manifest.id.as_ref() != Some(plugin_id) {
                issues.push(format!("Manifest ID mismatch for plugin: {}", plugin_id));
            }
            
            // Prüfe Script-Existenz
            let script_path = plugin.path.join(&plugin.manifest.main_script);
            if !script_path.exists() {
                issues.push(format!("Main script not found for plugin {}: {}", plugin_id, script_path.display()));
            }
        }
        
        issues
    }

    /// Hole Registry-Zusammenfassung
    pub fn get_registry_summary(&mut self) -> String {
        let stats = self.get_stats();
        
        format!(
            "📋 PLUGIN REGISTRY SUMMARY\n\
             ========================\n\
             Total Plugins: {}\n\
             Enabled: {} | Loaded: {} | Failed: {}\n\
             Total Size: {}\n\
             Top Permissions: {}\n\
             Top Authors: {}",
            stats.total_plugins,
            stats.enabled_plugins,
            stats.loaded_plugins,
            stats.failed_plugins,
            format_bytes(stats.total_size),
            format_top_items(&stats.permissions_count, 3),
            format_top_items(&stats.author_distribution, 3)
        )
    }

    // Getter
    pub fn get_config(&self) -> &PluginConfig {
        &self.config
    }

    pub fn get_plugin_count(&self) -> usize {
        self.plugins.len()
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}

/// Formatiere Bytes in menschenlesbares Format
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Formatiere Top-Items aus HashMap
fn format_top_items(items: &HashMap<String, usize>, limit: usize) -> String {
    let mut sorted_items: Vec<_> = items.iter().collect();
    sorted_items.sort_by(|a, b| b.1.cmp(a.1));
    
    sorted_items.iter()
        .take(limit)
        .map(|(key, count)| format!("{} ({})", key, count))
        .collect::<Vec<_>>()
        .join(", ")
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new(PluginConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::types::PluginManifest;
    use std::path::PathBuf;

    fn create_test_plugin(id: &str, enabled: bool) -> PluginInfo {
        PluginInfo {
            id: id.to_string(),
            path: PathBuf::from(format!("/test/{}", id)),
            manifest: PluginManifest {
                id: Some(id.to_string()),
                name: format!("Test Plugin {}", id),
                version: "1.0.0".to_string(),
                description: "Test plugin description".to_string(),
                author: "Test Author".to_string(),
                homepage: None,
                main_script: "main.js".to_string(),
                permissions: vec!["network".to_string(), "storage".to_string()],
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
            },
            loaded: false,
            load_error: None,
        }
    }

    #[test]
    fn test_registry_creation() {
        let registry = PluginRegistry::default();
        assert!(registry.is_empty());
        assert_eq!(registry.get_plugin_count(), 0);
    }

    #[test]
    fn test_plugin_registration() {
        let mut registry = PluginRegistry::default();
        let plugin = create_test_plugin("test-plugin", true);
        
        assert!(registry.register_plugin(plugin).is_ok());
        assert_eq!(registry.get_plugin_count(), 1);
        assert!(registry.contains_plugin("test-plugin"));
        
        // Test duplicate registration
        let duplicate_plugin = create_test_plugin("test-plugin", true);
        assert!(registry.register_plugin(duplicate_plugin).is_err());
    }

    #[test]
    fn test_plugin_unregistration() {
        let mut registry = PluginRegistry::default();
        let plugin = create_test_plugin("test-plugin", true);
        
        registry.register_plugin(plugin).unwrap();
        assert_eq!(registry.get_plugin_count(), 1);
        
        let removed_plugin = registry.unregister_plugin("test-plugin").unwrap();
        assert_eq!(removed_plugin.id, "test-plugin");
        assert_eq!(registry.get_plugin_count(), 0);
        
        // Test removing non-existent plugin
        assert!(registry.unregister_plugin("non-existent").is_err());
    }

    #[test]
    fn test_plugin_filtering() {
        let mut registry = PluginRegistry::default();
        
        let mut plugin1 = create_test_plugin("plugin1", true);
        plugin1.loaded = true;
        let plugin2 = create_test_plugin("plugin2", false);
        
        registry.register_plugin(plugin1).unwrap();
        registry.register_plugin(plugin2).unwrap();
        
        // Test enabled filter
        let enabled_plugins = registry.get_plugins_by_status(Some(true), None);
        assert_eq!(enabled_plugins.len(), 1);
        assert_eq!(enabled_plugins[0].id, "plugin1");
        
        // Test loaded filter
        let loaded_plugins = registry.get_plugins_by_status(None, Some(true));
        assert_eq!(loaded_plugins.len(), 1);
        assert_eq!(loaded_plugins[0].id, "plugin1");
        
        // Test combined filter
        let enabled_loaded = registry.get_plugins_by_status(Some(true), Some(true));
        assert_eq!(enabled_loaded.len(), 1);
    }

    #[test]
    fn test_plugin_search() {
        let mut registry = PluginRegistry::default();
        
        let plugin1 = create_test_plugin("search-test", true);
        let plugin2 = create_test_plugin("other-plugin", true);
        
        registry.register_plugin(plugin1).unwrap();
        registry.register_plugin(plugin2).unwrap();
        
        let search_results = registry.search_plugins("search");
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].id, "search-test");
        
        let author_results = registry.search_plugins("Test Author");
        assert_eq!(author_results.len(), 2);
    }

    #[test]
    fn test_stats_calculation() {
        let mut registry = PluginRegistry::default();
        
        let mut plugin1 = create_test_plugin("plugin1", true);
        plugin1.loaded = true;
        let plugin2 = create_test_plugin("plugin2", false);
        
        registry.register_plugin(plugin1).unwrap();
        registry.register_plugin(plugin2).unwrap();
        
        let stats = registry.get_stats();
        assert_eq!(stats.total_plugins, 2);
        assert_eq!(stats.enabled_plugins, 1);
        assert_eq!(stats.loaded_plugins, 1);
        assert_eq!(stats.failed_plugins, 0);
        
        // Test permissions count
        assert_eq!(stats.permissions_count.get("network"), Some(&2));
        assert_eq!(stats.permissions_count.get("storage"), Some(&2));
    }

    #[test]
    fn test_registry_export_import() {
        let mut registry = PluginRegistry::default();
        let plugin = create_test_plugin("test-plugin", true);
        
        registry.register_plugin(plugin).unwrap();
        
        // Export
        let exported = registry.export_registry().unwrap();
        assert!(exported.contains("test-plugin"));
        
        // Import to new registry
        let mut new_registry = PluginRegistry::default();
        let imported_count = new_registry.import_registry(&exported).unwrap();
        
        assert_eq!(imported_count, 1);
        assert!(new_registry.contains_plugin("test-plugin"));
    }

    #[test]
    fn test_registry_validation() {
        let mut registry = PluginRegistry::default();
        let mut plugin = create_test_plugin("test-plugin", true);
        
        // Create inconsistent plugin
        plugin.manifest.id = Some("different-id".to_string());
        registry.register_plugin(plugin).unwrap();
        
        let issues = registry.validate_registry();
        assert!(!issues.is_empty());
        assert!(issues[0].contains("mismatch"));
    }
}