// 🔌 Extension Manager für ZAKYX Browser
// Verwaltet Browser-Erweiterungen, Plugins und Add-ons

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub enabled: bool,
    pub script_path: PathBuf,
    pub permissions: Vec<String>,
    pub install_date: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub size: u64,
    pub category: ExtensionCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtensionCategory {
    AdBlocker,
    Productivity,
    Security,
    Developer,
    Social,
    Entertainment,
    Utility,
    Other,
}

impl ExtensionCategory {
    pub fn to_string(&self) -> String {
        match self {
            ExtensionCategory::AdBlocker => "🚫 Ad Blocker".to_string(),
            ExtensionCategory::Productivity => "⚡ Produktivität".to_string(),
            ExtensionCategory::Security => "🔒 Sicherheit".to_string(),
            ExtensionCategory::Developer => "💻 Entwickler".to_string(),
            ExtensionCategory::Social => "👥 Social".to_string(),
            ExtensionCategory::Entertainment => "🎮 Entertainment".to_string(),
            ExtensionCategory::Utility => "🔧 Utility".to_string(),
            ExtensionCategory::Other => "📦 Andere".to_string(),
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            ExtensionCategory::AdBlocker => "🚫",
            ExtensionCategory::Productivity => "⚡",
            ExtensionCategory::Security => "🔒",
            ExtensionCategory::Developer => "💻",
            ExtensionCategory::Social => "👥",
            ExtensionCategory::Entertainment => "🎮",
            ExtensionCategory::Utility => "🔧",
            ExtensionCategory::Other => "📦",
        }
    }
}

impl Extension {
    pub fn new(name: String, version: String, script_path: PathBuf) -> Self {
        let id = format!("ext_{}", Utc::now().timestamp());
        
        Self {
            id,
            name,
            version,
            description: String::new(),
            author: "Unknown".to_string(),
            enabled: false,
            script_path,
            permissions: Vec::new(),
            install_date: Utc::now(),
            last_updated: Utc::now(),
            size: 0,
            category: ExtensionCategory::Other,
        }
    }

    /// Prüfe ob Extension aktiv ist
    pub fn is_active(&self) -> bool {
        self.enabled && self.script_path.exists()
    }

    /// Hole Extension-Größe
    pub fn calculate_size(&mut self) -> Result<u64> {
        if self.script_path.exists() {
            let metadata = std::fs::metadata(&self.script_path)?;
            self.size = metadata.len();
            Ok(self.size)
        } else {
            Ok(0)
        }
    }

    /// Formatiere Größe
    pub fn format_size(&self) -> String {
        format_bytes(self.size)
    }

    /// Prüfe Berechtigungen
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }

    /// Füge Berechtigung hinzu
    pub fn add_permission(&mut self, permission: String) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
        }
    }

    /// Entferne Berechtigung
    pub fn remove_permission(&mut self, permission: &str) {
        self.permissions.retain(|p| p != permission);
    }
}

pub struct ExtensionManager {
    extensions: HashMap<String, Extension>,
    extensions_directory: PathBuf,
    enabled_count: usize,
    data_file: PathBuf,
}

impl ExtensionManager {
    pub fn new() -> Result<Self> {
        let extensions_directory = std::env::current_dir()?.join("extensions");
        let data_file = extensions_directory.join("extensions.json");
        
        // Erstelle Extensions-Verzeichnis falls nicht vorhanden
        if !extensions_directory.exists() {
            std::fs::create_dir_all(&extensions_directory)?;
        }

        let mut manager = Self {
            extensions: HashMap::new(),
            extensions_directory,
            enabled_count: 0,
            data_file,
        };

        // Lade Extensions und erstelle Standard-Extensions
        manager.load_extensions()?;
        manager.create_default_extensions()?;

        Ok(manager)
    }

    /// Erstelle Standard-Extensions
    fn create_default_extensions(&mut self) -> Result<()> {
        if self.extensions.is_empty() {
            // Ad Blocker Extension
            let mut ad_blocker = Extension::new(
                "ZAKYX Ad Blocker".to_string(),
                "1.0.0".to_string(),
                self.extensions_directory.join("ad_blocker.js")
            );
            ad_blocker.description = "Blockiert Werbung und Tracker".to_string();
            ad_blocker.author = "ZAKYX Team".to_string();
            ad_blocker.category = ExtensionCategory::AdBlocker;
            ad_blocker.permissions = vec![
                "webRequest".to_string(),
                "webRequestBlocking".to_string(),
                "storage".to_string(),
            ];
            ad_blocker.enabled = true;

            // Developer Tools Extension
            let mut dev_tools = Extension::new(
                "Developer Console".to_string(),
                "1.0.0".to_string(),
                self.extensions_directory.join("dev_console.js")
            );
            dev_tools.description = "Erweiterte Entwickler-Tools".to_string();
            dev_tools.author = "ZAKYX Team".to_string();
            dev_tools.category = ExtensionCategory::Developer;
            dev_tools.permissions = vec![
                "debugger".to_string(),
                "tabs".to_string(),
            ];

            // Privacy Guard Extension
            let mut privacy_guard = Extension::new(
                "Privacy Guard".to_string(),
                "1.0.0".to_string(),
                self.extensions_directory.join("privacy_guard.js")
            );
            privacy_guard.description = "Schutz der Privatsphäre".to_string();
            privacy_guard.author = "ZAKYX Team".to_string();
            privacy_guard.category = ExtensionCategory::Security;
            privacy_guard.permissions = vec![
                "privacy".to_string(),
                "cookies".to_string(),
                "storage".to_string(),
            ];

            self.add_extension(ad_blocker)?;
            self.add_extension(dev_tools)?;
            self.add_extension(privacy_guard)?;

            println!("📦 Created {} default extensions", self.extensions.len());
        }

        Ok(())
    }

    /// Füge Extension hinzu
    pub fn add_extension(&mut self, mut extension: Extension) -> Result<String> {
        // Berechne Größe
        let _ = extension.calculate_size();
        
        let extension_id = extension.id.clone();
        
        if extension.enabled {
            self.enabled_count += 1;
        }
        
        self.extensions.insert(extension_id.clone(), extension);
        self.save_extensions()?;
        
        println!("📦 Extension added: {}", extension_id);
        Ok(extension_id)
    }

    /// Entferne Extension
    pub fn remove_extension(&mut self, extension_id: &str) -> Result<()> {
        if let Some(extension) = self.extensions.remove(extension_id) {
            if extension.enabled {
                self.enabled_count = self.enabled_count.saturating_sub(1);
            }
            
            // Lösche Extension-Datei (optional)
            if extension.script_path.exists() {
                let _ = std::fs::remove_file(&extension.script_path);
            }
            
            self.save_extensions()?;
            println!("🗑️ Extension removed: {}", extension.name);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Extension not found: {}", extension_id))
        }
    }

    /// Toggle Extension
    pub fn toggle_extension(&mut self, extension_id: &str) -> Result<bool> {
        if let Some(extension) = self.extensions.get_mut(extension_id) {
            extension.enabled = !extension.enabled;
            extension.last_updated = Utc::now();
            
            let enabled = extension.enabled;
            let name = extension.name.clone();
            
            if enabled {
                self.enabled_count += 1;
                println!("✅ Extension enabled: {}", name);
            } else {
                self.enabled_count = self.enabled_count.saturating_sub(1);
                println!("❌ Extension disabled: {}", name);
            }
            
            self.save_extensions()?;
            Ok(enabled)
        } else {
            Err(anyhow::anyhow!("Extension not found: {}", extension_id))
        }
    }

    /// Installiere Extension aus Datei
    pub async fn install_extension(&mut self, extension_file: PathBuf) -> Result<String> {
        if !extension_file.exists() {
            return Err(anyhow::anyhow!("Extension file not found: {:?}", extension_file));
        }

        // Lese Extension-Metadaten (vereinfacht)
        let filename = extension_file.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let target_path = self.extensions_directory.join(extension_file.file_name().unwrap());
        
        // Kopiere Extension-Datei
        fs::copy(&extension_file, &target_path).await?;

        // Erstelle Extension-Eintrag
        let mut extension = Extension::new(
            filename.clone(),
            "1.0.0".to_string(),
            target_path
        );
        extension.description = format!("Installed extension: {}", filename);
        extension.author = "External".to_string();
        
        let extension_id = self.add_extension(extension)?;
        println!("📦 Extension installed: {}", filename);
        
        Ok(extension_id)
    }

    /// Aktualisiere Extension
    pub fn update_extension(&mut self, extension_id: &str, new_version: String) -> Result<()> {
        if let Some(extension) = self.extensions.get_mut(extension_id) {
            extension.version = new_version;
            extension.last_updated = Utc::now();
            let _ = extension.calculate_size();
            
            let name = extension.name.clone();
            
            self.save_extensions()?;
            println!("🔄 Extension updated: {}", name);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Extension not found: {}", extension_id))
        }
    }

    /// Suche Extensions
    pub fn search_extensions(&self, query: &str) -> Vec<&Extension> {
        let query_lower = query.to_lowercase();
        self.extensions.values()
            .filter(|ext| {
                ext.name.to_lowercase().contains(&query_lower) ||
                ext.description.to_lowercase().contains(&query_lower) ||
                ext.author.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Hole Extensions nach Kategorie
    pub fn get_extensions_by_category(&self, category: &ExtensionCategory) -> Vec<&Extension> {
        self.extensions.values()
            .filter(|ext| std::mem::discriminant(&ext.category) == std::mem::discriminant(category))
            .collect()
    }

    /// Hole aktivierte Extensions
    pub fn get_enabled_extensions(&self) -> Vec<&Extension> {
        self.extensions.values()
            .filter(|ext| ext.enabled)
            .collect()
    }

    /// Hole Extension-Statistiken
    pub fn get_stats(&self) -> ExtensionStats {
        let mut stats = ExtensionStats::default();
        stats.total_extensions = self.extensions.len();
        stats.enabled_extensions = self.enabled_count;

        let mut category_counts: HashMap<String, usize> = HashMap::new();
        let mut total_size = 0u64;

        for extension in self.extensions.values() {
            let category_name = extension.category.to_string();
            *category_counts.entry(category_name).or_insert(0) += 1;
            total_size += extension.size;
        }

        stats.category_counts = category_counts;
        stats.total_size = total_size;
        stats
    }

    /// Lade Extensions von Datei
    fn load_extensions(&mut self) -> Result<()> {
        if !self.data_file.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(&self.data_file)?;
        if content.trim().is_empty() {
            return Ok(());
        }

        let data: serde_json::Value = serde_json::from_str(&content)?;
        
        if let Some(extensions_obj) = data.get("extensions") {
            self.extensions = serde_json::from_value(extensions_obj.clone())?;
            
            // Zähle aktivierte Extensions
            self.enabled_count = self.extensions.values()
                .filter(|ext| ext.enabled)
                .count();
        }

        println!("📥 Loaded {} extensions", self.extensions.len());
        Ok(())
    }

    /// Speichere Extensions in Datei
    fn save_extensions(&self) -> Result<()> {
        let data = serde_json::json!({
            "extensions": self.extensions,
            "saved_at": Utc::now().to_rfc3339(),
            "version": "1.0"
        });

        let content = serde_json::to_string_pretty(&data)?;
        std::fs::write(&self.data_file, content)?;
        
        Ok(())
    }

    /// Hole Extensions-Display
    pub fn get_extensions_display(&self) -> Vec<String> {
        let mut result = vec![
            "🔌 EXTENSION MANAGER".to_string(),
            "====================".to_string(),
            "".to_string(),
        ];

        let stats = self.get_stats();
        result.extend(vec![
            format!("📊 Statistiken:"),
            format!("   • Gesamt: {} Extensions", stats.total_extensions),
            format!("   • Aktiviert: {} Extensions", stats.enabled_extensions),
            format!("   • Gesamtgröße: {}", format_bytes(stats.total_size)),
            "".to_string(),
            format!("📂 Kategorien:"),
        ]);

        for (category, count) in &stats.category_counts {
            result.push(format!("   • {}: {} Extensions", category, count));
        }
        result.push("".to_string());

        if self.extensions.is_empty() {
            result.push("📭 Keine Extensions installiert".to_string());
        } else {
            result.push("📋 INSTALLIERTE EXTENSIONS:".to_string());
            for extension in self.extensions.values() {
                let status = if extension.enabled {
                    "✅ Aktiviert"
                } else {
                    "❌ Deaktiviert"
                };

                result.push(format!("{} {}", extension.category.icon(), extension.name));
                result.push(format!("   ID: {}", extension.id));
                result.push(format!("   Version: {}", extension.version));
                result.push(format!("   Status: {}", status));
                result.push(format!("   Autor: {}", extension.author));
                result.push(format!("   Größe: {}", extension.format_size()));
                result.push(format!("   Kategorie: {}", extension.category.to_string()));
                
                if !extension.permissions.is_empty() {
                    result.push(format!("   Berechtigungen: {}", extension.permissions.join(", ")));
                }
                
                if !extension.description.is_empty() {
                    result.push(format!("   Beschreibung: {}", extension.description));
                }
                
                result.push(format!("   Installiert: {}", extension.install_date.format("%d.%m.%Y %H:%M")));
                result.push("".to_string());
            }
        }

        result
    }

    // Getter
    pub fn get_extensions(&self) -> &HashMap<String, Extension> {
        &self.extensions
    }

    pub fn get_extension(&self, extension_id: &str) -> Option<&Extension> {
        self.extensions.get(extension_id)
    }

    pub fn get_extension_count(&self) -> usize {
        self.extensions.len()
    }

    pub fn get_enabled_count(&self) -> usize {
        self.enabled_count
    }

    pub fn get_extensions_directory(&self) -> &PathBuf {
        &self.extensions_directory
    }
}

#[derive(Debug, Default)]
pub struct ExtensionStats {
    pub total_extensions: usize,
    pub enabled_extensions: usize,
    pub category_counts: HashMap<String, usize>,
    pub total_size: u64,
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

impl Default for ExtensionManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            extensions: HashMap::new(),
            extensions_directory: PathBuf::from("extensions"),
            enabled_count: 0,
            data_file: PathBuf::from("extensions/extensions.json"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_creation() {
        let extension = Extension::new(
            "Test Extension".to_string(),
            "1.0.0".to_string(),
            PathBuf::from("test.js")
        );
        
        assert_eq!(extension.name, "Test Extension");
        assert_eq!(extension.version, "1.0.0");
        assert!(!extension.enabled);
        assert!(matches!(extension.category, ExtensionCategory::Other));
    }

    #[test]
    fn test_extension_permissions() {
        let mut extension = Extension::new(
            "Test".to_string(),
            "1.0.0".to_string(),
            PathBuf::from("test.js")
        );
        
        extension.add_permission("storage".to_string());
        extension.add_permission("tabs".to_string());
        
        assert!(extension.has_permission("storage"));
        assert!(extension.has_permission("tabs"));
        assert!(!extension.has_permission("cookies"));
        
        extension.remove_permission("storage");
        assert!(!extension.has_permission("storage"));
    }

    #[test]
    fn test_extension_manager() {
        let mut manager = ExtensionManager::new().unwrap();
        
        let extension = Extension::new(
            "Test Extension".to_string(),
            "1.0.0".to_string(),
            PathBuf::from("test.js")
        );
        
        let id = manager.add_extension(extension).unwrap();
        assert_eq!(manager.get_extension_count(), 4); // 3 default + 1 test
        
        let enabled = manager.toggle_extension(&id).unwrap();
        assert!(enabled);
        assert_eq!(manager.get_enabled_count(), 2); // 1 default enabled + 1 test
        
        manager.remove_extension(&id).unwrap();
        assert_eq!(manager.get_extension_count(), 3); // Back to 3 default
    }

    #[test]
    fn test_extension_search() {
        let mut manager = ExtensionManager::new().unwrap();
        let results = manager.search_extensions("ad");
        
        // Should find the default Ad Blocker extension
        assert!(!results.is_empty());
        assert!(results[0].name.to_lowercase().contains("ad"));
    }

    #[test]
    fn test_extension_categories() {
        let manager = ExtensionManager::new().unwrap();
        let ad_blockers = manager.get_extensions_by_category(&ExtensionCategory::AdBlocker);
        
        assert!(!ad_blockers.is_empty());
        assert!(matches!(ad_blockers[0].category, ExtensionCategory::AdBlocker));
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
    }
}