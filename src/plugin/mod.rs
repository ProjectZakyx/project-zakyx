// 🔌 PLUGIN MODULE
// Hauptmodul für das Plugin-System
// Copyright © 2024 ZAKYX Browser Team

#![allow(dead_code)] // Plugin system API - comprehensive API kept for extensibility

use chrono::Datelike;

pub mod types;
pub mod validator;
pub mod discovery;
pub mod loader;
pub mod management;
pub mod manager;

// Re-exports für einfache Verwendung
pub use types::{
    PluginManifest,
    PluginInfo, PluginConfig,
    ALLOWED_PERMISSIONS
};

pub use manager::PluginManager;

// Convenience-Funktionen
impl PluginManager {
    /// Führt eine vollständige Plugin-Initialisierung durch
    pub fn quick_setup() -> Result<Self, String> {
        let mut manager = PluginManager::new();
        manager.initialize()?;
        Ok(manager)
    }
    
    /// Führt eine Plugin-Initialisierung mit benutzerdefinierten Einstellungen durch
    pub fn setup_with_directory(plugins_dir: std::path::PathBuf) -> Result<Self, String> {
        let config = PluginConfig {
            plugins_directory: plugins_dir,
            ..Default::default()
        };
        
        let mut manager = PluginManager::with_config(config);
        manager.initialize()?;
        Ok(manager)
    }
}

// Hilfsfunktionen für Plugin-Operationen
pub fn create_basic_plugin_manifest(
    name: &str,
    version: &str,
    description: &str,
    author: &str,
    main_script: &str,
    permissions: Vec<String>,
) -> PluginManifest {
    PluginManifest {
        id: Some(name.to_lowercase().replace(' ', "-")),
        name: name.to_string(),
        version: version.to_string(),
        description: description.to_string(),
        author: author.to_string(),
        homepage: None,
        main_script: main_script.to_string(),
        permissions,
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

/// Validiert eine Plugin-ID nach den Standard-Regeln
pub fn validate_plugin_id(plugin_id: &str) -> Result<(), String> {
    if plugin_id.is_empty() {
        return Err("Plugin ID cannot be empty".to_string());
    }
    
    if plugin_id.len() > 50 {
        return Err("Plugin ID cannot be longer than 50 characters".to_string());
    }
    
    if !plugin_id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err("Plugin ID can only contain alphanumeric characters, hyphens, and underscores".to_string());
    }
    
    if plugin_id.starts_with('-') || plugin_id.ends_with('-') {
        return Err("Plugin ID cannot start or end with a hyphen".to_string());
    }
    
    Ok(())
}

/// Gibt Standard-Berechtigungen für verschiedene Plugin-Typen zurück
pub fn get_default_permissions(plugin_type: &str) -> Vec<String> {
    match plugin_type {
        "basic" => vec![],
        "network" => vec!["network".to_string()],
        "storage" => vec!["storage".to_string()],
        "browser" => vec!["tabs".to_string(), "bookmarks".to_string(), "history".to_string()],
        "advanced" => vec![
            "network".to_string(),
            "storage".to_string(),
            "tabs".to_string(),
            "bookmarks".to_string(),
            "history".to_string(),
            "webRequest".to_string(),
        ],
        "full" => ALLOWED_PERMISSIONS.iter().map(|&s| s.to_string()).collect(),
        _ => vec![],
    }
}

/// Erstellt ein Standard-Plugin-Verzeichnis mit Boilerplate-Code
pub fn create_plugin_template(
    plugins_dir: &std::path::Path,
    plugin_id: &str,
    manifest: &PluginManifest,
) -> Result<std::path::PathBuf, String> {
    validate_plugin_id(plugin_id)?;
    
    let plugin_dir = plugins_dir.join(plugin_id);
    
    if plugin_dir.exists() {
        return Err(format!("Plugin directory already exists: {}", plugin_id));
    }
    
    std::fs::create_dir_all(&plugin_dir)
        .map_err(|e| format!("Failed to create plugin directory: {}", e))?;
    
    // Erstelle plugin.json
    let manifest_content = serde_json::to_string_pretty(manifest)
        .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
    
    std::fs::write(plugin_dir.join("plugin.json"), manifest_content)
        .map_err(|e| format!("Failed to create plugin.json: {}", e))?;
    
    // Erstelle Hauptskript mit Boilerplate
    let main_script_content = format!(
        r#"// {} Plugin
// Version: {}
// Author: {}

// Plugin-Initialisierung
console.log('Plugin "{}" wird geladen...');

// Hauptfunktionalität hier implementieren
function initialize() {{
    console.log('Plugin "{}" wurde initialisiert.');
    
    // Beispiel: Event-Listener hinzufügen
    // document.addEventListener('DOMContentLoaded', function() {{
    //     console.log('DOM ist bereit');
    // }});
}}

// Plugin-Cleanup
function cleanup() {{
    console.log('Plugin "{}" wird entladen...');
    
    // Hier aufräumen: Event-Listener entfernen, Timer stoppen, etc.
}}

// Automatische Initialisierung
if (typeof window !== 'undefined') {{
    initialize();
}}

// Exportiere Funktionen für Plugin-API
if (typeof module !== 'undefined' && module.exports) {{
    module.exports = {{
        initialize,
        cleanup,
        name: '{}',
        version: '{}'
    }};
}}
"#,
        manifest.name,
        manifest.version,
        manifest.author,
        manifest.name,
        manifest.name,
        manifest.name,
        manifest.name,
        manifest.version
    );
    
    std::fs::write(plugin_dir.join(&manifest.main_script), main_script_content)
        .map_err(|e| format!("Failed to create main script: {}", e))?;
    
    // Erstelle README.md
    let readme_content = format!(
        r#"# {} Plugin

## Beschreibung
{}

## Version
{}

## Autor
{}

## Installation
        1. Kopiere diesen Ordner in das Plugin-Verzeichnis des ZAKYX Browsers
2. Starte den Browser neu oder lade die Plugins neu
3. Aktiviere das Plugin in den Einstellungen

## Konfiguration
Das Plugin kann über die `plugin.json` Datei konfiguriert werden.

## Berechtigungen
Dieses Plugin benötigt die folgenden Berechtigungen:
{}

## Entwicklung
- Hauptskript: `{}`
- Plugin-Manifest: `plugin.json`

## Lizenz
Copyright © {} {}
"#,
        manifest.name,
        manifest.description,
        manifest.version,
        manifest.author,
        manifest.permissions.iter().map(|p| format!("- {}", p)).collect::<Vec<_>>().join("\n"),
        manifest.main_script,
        chrono::Utc::now().year(),
        manifest.author
    );
    
    std::fs::write(plugin_dir.join("README.md"), readme_content)
        .map_err(|e| format!("Failed to create README.md: {}", e))?;
    
    Ok(plugin_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    
    fn create_test_dir() -> std::path::PathBuf {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let test_dir = std::env::temp_dir().join(format!("zakyx_test_mod_{}_{}", std::process::id(), nanos));
        if test_dir.exists() {
            let _ = fs::remove_dir_all(&test_dir);
        }
        fs::create_dir_all(&test_dir).unwrap();
        test_dir
    }
    
    #[test]
    fn test_quick_setup() {
        let test_dir = create_test_dir();
        let manager = PluginManager::setup_with_directory(test_dir.clone());
        
        assert!(manager.is_ok());
        
        let manager = manager.unwrap();
        assert_eq!(manager.get_config().plugins_directory, test_dir);
        
        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
    
    #[test]
    fn test_create_basic_plugin_manifest() {
        let manifest = create_basic_plugin_manifest(
            "Test Plugin",
            "1.0.0",
            "A test plugin",
            "Test Author",
            "main.js",
            vec!["network".to_string()],
        );
        
        assert_eq!(manifest.name, "Test Plugin");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.description, "A test plugin");
        assert_eq!(manifest.author, "Test Author");
        assert_eq!(manifest.main_script, "main.js");
        assert_eq!(manifest.permissions, vec!["network".to_string()]);
        assert_eq!(manifest.api_version, "1.0");
        assert!(manifest.enabled);
        assert_eq!(manifest.id, Some("test-plugin".to_string()));
    }
    
    #[test]
    fn test_validate_plugin_id() {
        assert!(validate_plugin_id("valid-plugin").is_ok());
        assert!(validate_plugin_id("valid_plugin").is_ok());
        assert!(validate_plugin_id("validplugin123").is_ok());
        
        assert!(validate_plugin_id("").is_err());
        assert!(validate_plugin_id("-invalid").is_err());
        assert!(validate_plugin_id("invalid-").is_err());
        assert!(validate_plugin_id("invalid plugin").is_err());
        assert!(validate_plugin_id("invalid@plugin").is_err());
        
        let long_id = "a".repeat(51);
        assert!(validate_plugin_id(&long_id).is_err());
    }
    
    #[test]
    fn test_get_default_permissions() {
        assert_eq!(get_default_permissions("basic"), Vec::<String>::new());
        assert_eq!(get_default_permissions("network"), vec!["network".to_string()]);
        assert_eq!(get_default_permissions("storage"), vec!["storage".to_string()]);
        
        let browser_perms = get_default_permissions("browser");
        assert!(browser_perms.contains(&"tabs".to_string()));
        assert!(browser_perms.contains(&"bookmarks".to_string()));
        assert!(browser_perms.contains(&"history".to_string()));
        
        let full_perms = get_default_permissions("full");
        assert_eq!(full_perms.len(), ALLOWED_PERMISSIONS.len());
        
        assert_eq!(get_default_permissions("unknown"), Vec::<String>::new());
    }
    
    #[test]
    fn test_create_plugin_template() {
        let test_dir = create_test_dir();
        let manifest = create_basic_plugin_manifest(
            "Test Plugin",
            "1.0.0",
            "A test plugin",
            "Test Author",
            "main.js",
            vec!["network".to_string()],
        );
        
        let plugin_dir = create_plugin_template(&test_dir, "test-plugin", &manifest).unwrap();
        
        assert!(plugin_dir.exists());
        assert!(plugin_dir.join("plugin.json").exists());
        assert!(plugin_dir.join("main.js").exists());
        assert!(plugin_dir.join("README.md").exists());
        
        // Prüfe dass das Manifest korrekt geschrieben wurde
        let manifest_content = fs::read_to_string(plugin_dir.join("plugin.json")).unwrap();
        let loaded_manifest: PluginManifest = serde_json::from_str(&manifest_content).unwrap();
        assert_eq!(loaded_manifest.name, "Test Plugin");
        
        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
} 
