// 🔍 Plugin Operations für ZAKYX Browser
// High-Level-Operationen für Plugin-Management: List, Search, Stats, Info

use std::collections::HashMap;
use crate::plugin::types::{PluginInfo, PluginStats};
use crate::plugin::management::registry::PluginRegistry;

/// Plugin-Operations-Manager
#[derive(Debug)]
pub struct PluginOperationsManager {
    registry: PluginRegistry,
}

impl PluginOperationsManager {
    /// Erstellt einen neuen Operations-Manager
    pub fn new(registry: PluginRegistry) -> Self {
        Self { registry }
    }

    /// Hole alle Plugins
    pub fn list_all_plugins(&self) -> Vec<&PluginInfo> {
        self.registry.get_all_plugins().values().collect()
    }

    /// Hole aktivierte Plugins
    pub fn list_enabled_plugins(&self) -> Vec<&PluginInfo> {
        self.registry.get_plugins_by_status(Some(true), None)
    }

    /// Hole deaktivierte Plugins
    pub fn list_disabled_plugins(&self) -> Vec<&PluginInfo> {
        self.registry.get_plugins_by_status(Some(false), None)
    }

    /// Hole geladene Plugins
    pub fn list_loaded_plugins(&self) -> Vec<&PluginInfo> {
        self.registry.get_plugins_by_status(None, Some(true))
    }

    /// Hole nicht geladene Plugins
    pub fn list_unloaded_plugins(&self) -> Vec<&PluginInfo> {
        self.registry.get_plugins_by_status(None, Some(false))
    }

    /// Hole fehlgeschlagene Plugins
    pub fn list_failed_plugins(&self) -> Vec<&PluginInfo> {
        self.registry.get_all_plugins()
            .values()
            .filter(|plugin| plugin.load_error.is_some())
            .collect()
    }

    /// Suche Plugins
    pub fn search_plugins(&self, query: &str) -> Vec<&PluginInfo> {
        self.registry.search_plugins(query)
    }

    /// Suche Plugins nach Autor
    pub fn search_plugins_by_author(&self, author: &str) -> Vec<&PluginInfo> {
        let author_lower = author.to_lowercase();
        self.registry.get_all_plugins()
            .values()
            .filter(|plugin| plugin.manifest.author.to_lowercase().contains(&author_lower))
            .collect()
    }

    /// Suche Plugins nach Version
    pub fn search_plugins_by_version(&self, version: &str) -> Vec<&PluginInfo> {
        self.registry.get_all_plugins()
            .values()
            .filter(|plugin| plugin.manifest.version == version)
            .collect()
    }

    /// Hole Plugins mit bestimmter Berechtigung
    pub fn get_plugins_with_permission(&self, permission: &str) -> Vec<&PluginInfo> {
        self.registry.get_plugins_with_permission(permission)
    }

    /// Hole Plugin-Informationen
    pub fn get_plugin_info(&self, plugin_id: &str) -> Option<&PluginInfo> {
        self.registry.get_plugin(plugin_id)
    }

    /// Hole detaillierte Plugin-Informationen
    pub fn get_detailed_plugin_info(&self, plugin_id: &str) -> Option<DetailedPluginInfo> {
        self.registry.get_plugin(plugin_id).map(|plugin| {
            let mut detailed = DetailedPluginInfo {
                basic_info: plugin.clone(),
                file_count: 0,
                total_size: 0,
                dependencies: Vec::new(),
                dependents: Vec::new(),
                last_modified: None,
                security_score: 0,
            };

            // Berechne Dateigröße und -anzahl
            if let Ok(metadata) = std::fs::metadata(&plugin.path) {
                detailed.total_size = metadata.len();
                detailed.last_modified = metadata.modified().ok();
            }

            if let Ok(entries) = std::fs::read_dir(&plugin.path) {
                detailed.file_count = entries.count();
            }

            // Finde Abhängigkeiten
            if let Some(deps) = &plugin.manifest.dependencies {
                detailed.dependencies = deps.keys().cloned().collect();
            }

            // Finde abhängige Plugins
            detailed.dependents = self.find_dependent_plugins(plugin_id);

            // Berechne Sicherheits-Score
            detailed.security_score = self.calculate_security_score(plugin);

            detailed
        })
    }

    /// Hole Plugin-Statistiken
    pub fn get_plugin_stats(&mut self) -> &PluginStats {
        self.registry.get_stats()
    }

    /// Hole erweiterte Statistiken
    pub fn get_extended_stats(&mut self) -> ExtendedPluginStats {
        let basic_stats = self.registry.get_stats().clone();
        
        let plugins = self.registry.get_all_plugins();
        let mut extended = ExtendedPluginStats {
            basic: basic_stats.clone(),
            avg_permissions_per_plugin: 0.0,
            most_common_permissions: Vec::new(),
            plugin_size_distribution: HashMap::new(),
            load_success_rate: 0.0,
            top_authors: Vec::new(),
            version_compatibility: HashMap::new(),
        };

        if !plugins.is_empty() {
            // Durchschnittliche Berechtigungen pro Plugin
            let total_permissions: usize = plugins.values()
                .map(|p| p.manifest.permissions.len())
                .sum();
            extended.avg_permissions_per_plugin = total_permissions as f64 / plugins.len() as f64;

            // Häufigste Berechtigungen
            extended.most_common_permissions = basic_stats.permissions_count
                .iter()
                .map(|(perm, count)| (perm.clone(), *count))
                .collect::<Vec<_>>();
            extended.most_common_permissions.sort_by(|a, b| b.1.cmp(&a.1));
            extended.most_common_permissions.truncate(10);

            // Plugin-Größenverteilung
            for plugin in plugins.values() {
                if let Ok(metadata) = std::fs::metadata(&plugin.path) {
                    let size_category = categorize_size(metadata.len());
                    *extended.plugin_size_distribution.entry(size_category).or_insert(0) += 1;
                }
            }

            // Lade-Erfolgsrate
            let failed_plugins = plugins.values().filter(|p| p.load_error.is_some()).count();
            let attempted_loads = plugins.values().filter(|p| p.manifest.enabled).count();
            if attempted_loads > 0 {
                extended.load_success_rate = ((attempted_loads - failed_plugins) as f64 / attempted_loads as f64) * 100.0;
            }

            // Top-Autoren
            extended.top_authors = basic_stats.author_distribution
                .iter()
                .map(|(author, count)| (author.clone(), *count))
                .collect::<Vec<_>>();
            extended.top_authors.sort_by(|a, b| b.1.cmp(&a.1));
            extended.top_authors.truncate(10);

            // Versions-Kompatibilität
            for plugin in plugins.values() {
                let version_key = format!("v{}", plugin.manifest.api_version);
                *extended.version_compatibility.entry(version_key).or_insert(0) += 1;
            }
        }

        extended
    }

    /// Generiere Plugin-Report
    pub fn generate_plugin_report(&mut self) -> String {
        let stats = self.get_extended_stats();
        let _plugins = self.registry.get_all_plugins();
        
        let mut report = String::new();
        report.push_str("📊 ZAKYX BROWSER PLUGIN REPORT\n");
        report.push_str("==============================\n\n");

        // Basis-Statistiken
        report.push_str(&format!("📈 OVERVIEW:\n"));
        report.push_str(&format!("  Total Plugins: {}\n", stats.basic.total_plugins));
        report.push_str(&format!("  Enabled: {} ({:.1}%)\n", 
            stats.basic.enabled_plugins,
            if stats.basic.total_plugins > 0 { 
                (stats.basic.enabled_plugins as f64 / stats.basic.total_plugins as f64) * 100.0 
            } else { 0.0 }
        ));
        report.push_str(&format!("  Loaded: {} ({:.1}%)\n", 
            stats.basic.loaded_plugins,
            if stats.basic.enabled_plugins > 0 { 
                (stats.basic.loaded_plugins as f64 / stats.basic.enabled_plugins as f64) * 100.0 
            } else { 0.0 }
        ));
        report.push_str(&format!("  Failed: {}\n", stats.basic.failed_plugins));
        report.push_str(&format!("  Load Success Rate: {:.1}%\n\n", stats.load_success_rate));

        // Berechtigungen
        report.push_str("🔒 PERMISSIONS:\n");
        report.push_str(&format!("  Average per Plugin: {:.1}\n", stats.avg_permissions_per_plugin));
        report.push_str("  Most Common:\n");
        for (perm, count) in stats.most_common_permissions.iter().take(5) {
            report.push_str(&format!("    • {} ({})\n", perm, count));
        }
        report.push_str("\n");

        // Autoren
        report.push_str("👥 AUTHORS:\n");
        for (author, count) in stats.top_authors.iter().take(5) {
            report.push_str(&format!("  • {} ({} plugins)\n", author, count));
        }
        report.push_str("\n");

        // Größenverteilung
        report.push_str("📦 SIZE DISTRIBUTION:\n");
        for (size_cat, count) in &stats.plugin_size_distribution {
            report.push_str(&format!("  • {}: {} plugins\n", size_cat, count));
        }
        report.push_str("\n");

        // API-Versionen
        report.push_str("🔧 API COMPATIBILITY:\n");
        for (version, count) in &stats.version_compatibility {
            report.push_str(&format!("  • {}: {} plugins\n", version, count));
        }
        report.push_str("\n");

        // Problematische Plugins
        let failed_plugins = self.list_failed_plugins();
        if !failed_plugins.is_empty() {
            report.push_str("⚠️ FAILED PLUGINS:\n");
            for plugin in failed_plugins.iter().take(5) {
                if let Some(error) = &plugin.load_error {
                    report.push_str(&format!("  • {}: {}\n", plugin.id, error));
                }
            }
            report.push_str("\n");
        }

        // Empfehlungen
        report.push_str("💡 RECOMMENDATIONS:\n");
        if stats.load_success_rate < 90.0 {
            report.push_str("  • Review failed plugins and fix compatibility issues\n");
        }
        if stats.avg_permissions_per_plugin > 5.0 {
            report.push_str("  • Consider reviewing plugins with excessive permissions\n");
        }
        if stats.basic.total_plugins > 50 {
            report.push_str("  • Consider organizing plugins into categories\n");
        }

        report
    }

    /// Exportiere Plugin-Liste
    pub fn export_plugin_list(&self, format: ExportFormat) -> Result<String, String> {
        let plugins = self.registry.get_all_plugins();
        
        match format {
            ExportFormat::Json => {
                serde_json::to_string_pretty(plugins)
                    .map_err(|e| format!("JSON export failed: {}", e))
            }
            ExportFormat::Csv => {
                let mut csv = String::new();
                csv.push_str("ID,Name,Version,Author,Enabled,Loaded,Permissions\n");
                
                for plugin in plugins.values() {
                    csv.push_str(&format!(
                        "{},{},{},{},{},{},{}\n",
                        plugin.id,
                        plugin.manifest.name,
                        plugin.manifest.version,
                        plugin.manifest.author,
                        plugin.manifest.enabled,
                        plugin.loaded,
                        plugin.manifest.permissions.join(";")
                    ));
                }
                
                Ok(csv)
            }
            ExportFormat::Markdown => {
                let mut md = String::new();
                md.push_str("# Plugin List\n\n");
                md.push_str("| ID | Name | Version | Author | Status | Permissions |\n");
                md.push_str("|----|----|----|----|----|\n");
                
                for plugin in plugins.values() {
                    let status = match (plugin.manifest.enabled, plugin.loaded) {
                        (true, true) => "🟢 Loaded",
                        (true, false) => "🟡 Enabled",
                        (false, _) => "🔴 Disabled",
                    };
                    
                    md.push_str(&format!(
                        "| {} | {} | {} | {} | {} | {} |\n",
                        plugin.id,
                        plugin.manifest.name,
                        plugin.manifest.version,
                        plugin.manifest.author,
                        status,
                        plugin.manifest.permissions.join(", ")
                    ));
                }
                
                Ok(md)
            }
        }
    }

    /// Validiere Plugin-Abhängigkeiten
    pub fn validate_dependencies(&self) -> Vec<DependencyIssue> {
        let mut issues = Vec::new();
        let plugins = self.registry.get_all_plugins();
        
        for plugin in plugins.values() {
            if let Some(dependencies) = &plugin.manifest.dependencies {
                for (dep, _version) in dependencies {
                    if !plugins.contains_key(dep) {
                        issues.push(DependencyIssue {
                            plugin_id: plugin.id.clone(),
                            dependency: dep.clone(),
                            issue_type: DependencyIssueType::Missing,
                        });
                    } else if let Some(dep_plugin) = plugins.get(dep) {
                        if !dep_plugin.manifest.enabled {
                            issues.push(DependencyIssue {
                                plugin_id: plugin.id.clone(),
                                dependency: dep.clone(),
                                issue_type: DependencyIssueType::Disabled,
                            });
                        }
                    }
                }
            }
        }
        
        issues
    }

    /// Finde abhängige Plugins
    fn find_dependent_plugins(&self, plugin_id: &str) -> Vec<String> {
        self.registry.get_all_plugins()
            .values()
            .filter_map(|plugin| {
                if let Some(deps) = &plugin.manifest.dependencies {
                    if deps.contains_key(&plugin_id.to_string()) {
                        Some(plugin.id.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    /// Berechne Sicherheits-Score für Plugin
    fn calculate_security_score(&self, plugin: &PluginInfo) -> u8 {
        let mut score = 100u8;
        
        // Abzug für Berechtigungen
        let permission_penalty = match plugin.manifest.permissions.len() {
            0..=2 => 0,
            3..=5 => 10,
            6..=10 => 20,
            _ => 30,
        };
        score = score.saturating_sub(permission_penalty);
        
        // Abzug für kritische Berechtigungen
        let critical_permissions = ["filesystem", "network", "system", "native"];
        for critical in &critical_permissions {
            if plugin.manifest.permissions.iter().any(|p| p.contains(critical)) {
                score = score.saturating_sub(15);
            }
        }
        
        // Bonus für bekannte Autoren (vereinfacht)
        if plugin.manifest.author.contains("ZAKYX") {
            score = (score + 10).min(100);
        }
        
        score
    }

    // Getter für Registry-Zugriff
    pub fn get_registry(&self) -> &PluginRegistry {
        &self.registry
    }
}

/// Detaillierte Plugin-Informationen
#[derive(Debug, Clone)]
pub struct DetailedPluginInfo {
    pub basic_info: PluginInfo,
    pub file_count: usize,
    pub total_size: u64,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
    pub last_modified: Option<std::time::SystemTime>,
    pub security_score: u8,
}

/// Erweiterte Plugin-Statistiken
#[derive(Debug, Clone)]
pub struct ExtendedPluginStats {
    pub basic: PluginStats,
    pub avg_permissions_per_plugin: f64,
    pub most_common_permissions: Vec<(String, usize)>,
    pub plugin_size_distribution: HashMap<String, usize>,
    pub load_success_rate: f64,
    pub top_authors: Vec<(String, usize)>,
    pub version_compatibility: HashMap<String, usize>,
}

/// Export-Formate
#[derive(Debug, Clone)]
pub enum ExportFormat {
    Json,
    Csv,
    Markdown,
}

/// Abhängigkeits-Problem
#[derive(Debug, Clone)]
pub struct DependencyIssue {
    pub plugin_id: String,
    pub dependency: String,
    pub issue_type: DependencyIssueType,
}

/// Typ des Abhängigkeits-Problems
#[derive(Debug, Clone)]
pub enum DependencyIssueType {
    Missing,
    Disabled,
    VersionMismatch,
}

/// Kategorisiere Dateigröße
fn categorize_size(bytes: u64) -> String {
    match bytes {
        0..=1024 => "Tiny (<1KB)".to_string(),
        1025..=10240 => "Small (1-10KB)".to_string(),
        10241..=102400 => "Medium (10-100KB)".to_string(),
        102401..=1048576 => "Large (100KB-1MB)".to_string(),
        _ => "Very Large (>1MB)".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::types::{PluginManifest, PluginConfig};
    use std::path::PathBuf;

    fn create_test_plugin(id: &str, enabled: bool, loaded: bool) -> PluginInfo {
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
            },
            loaded,
            load_error: None,
        }
    }

    #[test]
    fn test_operations_manager_creation() {
        let registry = PluginRegistry::default();
        let manager = PluginOperationsManager::new(registry);
        
        assert_eq!(manager.list_all_plugins().len(), 0);
    }

    #[test]
    fn test_plugin_listing() {
        let mut registry = PluginRegistry::default();
        
        let plugin1 = create_test_plugin("plugin1", true, true);
        let plugin2 = create_test_plugin("plugin2", true, false);
        let plugin3 = create_test_plugin("plugin3", false, false);
        
        registry.register_plugin(plugin1).unwrap();
        registry.register_plugin(plugin2).unwrap();
        registry.register_plugin(plugin3).unwrap();
        
        let manager = PluginOperationsManager::new(registry);
        
        assert_eq!(manager.list_all_plugins().len(), 3);
        assert_eq!(manager.list_enabled_plugins().len(), 2);
        assert_eq!(manager.list_disabled_plugins().len(), 1);
        assert_eq!(manager.list_loaded_plugins().len(), 1);
        assert_eq!(manager.list_unloaded_plugins().len(), 2);
    }

    #[test]
    fn test_plugin_search() {
        let mut registry = PluginRegistry::default();
        
        let plugin1 = create_test_plugin("search-test", true, false);
        let plugin2 = create_test_plugin("other-plugin", true, false);
        
        registry.register_plugin(plugin1).unwrap();
        registry.register_plugin(plugin2).unwrap();
        
        let manager = PluginOperationsManager::new(registry);
        
        let search_results = manager.search_plugins("search");
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].id, "search-test");
        
        let author_results = manager.search_plugins_by_author("Test Author");
        assert_eq!(author_results.len(), 2);
    }

    #[test]
    fn test_detailed_plugin_info() {
        let mut registry = PluginRegistry::default();
        let plugin = create_test_plugin("test-plugin", true, false);
        
        registry.register_plugin(plugin).unwrap();
        let manager = PluginOperationsManager::new(registry);
        
        let detailed = manager.get_detailed_plugin_info("test-plugin");
        assert!(detailed.is_some());
        
        let info = detailed.unwrap();
        assert_eq!(info.basic_info.id, "test-plugin");
        assert!(info.security_score > 0);
    }

    #[test]
    fn test_export_formats() {
        let mut registry = PluginRegistry::default();
        let plugin = create_test_plugin("test-plugin", true, false);
        
        registry.register_plugin(plugin).unwrap();
        let manager = PluginOperationsManager::new(registry);
        
        // Test JSON export
        let json_export = manager.export_plugin_list(ExportFormat::Json);
        assert!(json_export.is_ok());
        assert!(json_export.unwrap().contains("test-plugin"));
        
        // Test CSV export
        let csv_export = manager.export_plugin_list(ExportFormat::Csv);
        assert!(csv_export.is_ok());
        assert!(csv_export.unwrap().contains("test-plugin"));
        
        // Test Markdown export
        let md_export = manager.export_plugin_list(ExportFormat::Markdown);
        assert!(md_export.is_ok());
        assert!(md_export.unwrap().contains("test-plugin"));
    }

    #[test]
    fn test_dependency_validation() {
        let mut registry = PluginRegistry::default();
        
        let mut plugin1 = create_test_plugin("plugin1", true, false);
        plugin1.manifest.dependencies = Some(vec!["missing-plugin".to_string()]);
        
        let mut plugin2 = create_test_plugin("plugin2", true, false);
        plugin2.manifest.dependencies = Some(vec!["plugin3".to_string()]);
        
        let plugin3 = create_test_plugin("plugin3", false, false); // Disabled
        
        registry.register_plugin(plugin1).unwrap();
        registry.register_plugin(plugin2).unwrap();
        registry.register_plugin(plugin3).unwrap();
        
        let manager = PluginOperationsManager::new(registry);
        let issues = manager.validate_dependencies();
        
        assert_eq!(issues.len(), 2);
        assert!(issues.iter().any(|i| matches!(i.issue_type, DependencyIssueType::Missing)));
        assert!(issues.iter().any(|i| matches!(i.issue_type, DependencyIssueType::Disabled)));
    }

    #[test]
    fn test_security_score_calculation() {
        let mut registry = PluginRegistry::default();
        let manager = PluginOperationsManager::new(registry);
        
        // Plugin mit wenigen Berechtigungen
        let safe_plugin = create_test_plugin("safe", true, false);
        let safe_score = manager.calculate_security_score(&safe_plugin);
        assert!(safe_score > 80);
        
        // Plugin mit vielen Berechtigungen
        let mut risky_plugin = create_test_plugin("risky", true, false);
        risky_plugin.manifest.permissions = vec![
            "filesystem".to_string(),
            "network".to_string(),
            "system".to_string(),
            "native".to_string(),
            "storage".to_string(),
            "tabs".to_string(),
        ];
        let risky_score = manager.calculate_security_score(&risky_plugin);
        assert!(risky_score < safe_score);
    }
}