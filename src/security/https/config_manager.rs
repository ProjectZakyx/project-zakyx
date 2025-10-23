// ⚙️ HTTPS Configuration Manager
// Verwaltet HTTPS-Einstellungen, Presets und Import/Export

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpsConfig {
    pub enabled: bool,
    pub enforce_strict: bool,
    pub hsts_enabled: bool,
    pub certificate_validation: bool,
    pub mixed_content_blocking: bool,
    pub allowed_http_domains: Vec<String>,
    pub custom_settings: HashMap<String, serde_json::Value>,
}

pub struct HttpsConfigManager {
    config: HttpsConfig,
    presets: HashMap<String, HttpsConfig>,
}

impl HttpsConfigManager {
    pub fn new() -> Self {
        let mut manager = Self {
            config: HttpsConfig::default(),
            presets: HashMap::new(),
        };
        
        manager.load_default_presets();
        manager
    }

    /// Lade Standard-Presets
    fn load_default_presets(&mut self) {
        // Strict Preset - Maximale Sicherheit
        self.presets.insert("strict".to_string(), HttpsConfig {
            enabled: true,
            enforce_strict: true,
            hsts_enabled: true,
            certificate_validation: true,
            mixed_content_blocking: true,
            allowed_http_domains: vec![
                "localhost".to_string(),
                "127.0.0.1".to_string(),
            ],
            custom_settings: HashMap::new(),
        });

        // Balanced Preset - Ausgewogen
        self.presets.insert("balanced".to_string(), HttpsConfig {
            enabled: true,
            enforce_strict: false,
            hsts_enabled: true,
            certificate_validation: true,
            mixed_content_blocking: true,
            allowed_http_domains: vec![
                "localhost".to_string(),
                "127.0.0.1".to_string(),
                "::1".to_string(),
            ],
            custom_settings: HashMap::new(),
        });

        // Minimal Preset - Grundschutz
        self.presets.insert("minimal".to_string(), HttpsConfig {
            enabled: true,
            enforce_strict: false,
            hsts_enabled: false,
            certificate_validation: false,
            mixed_content_blocking: false,
            allowed_http_domains: vec![
                "localhost".to_string(),
                "127.0.0.1".to_string(),
                "::1".to_string(),
                "192.168.0.0/16".to_string(),
                "10.0.0.0/8".to_string(),
            ],
            custom_settings: HashMap::new(),
        });

        // Development Preset - Entwicklung
        self.presets.insert("development".to_string(), HttpsConfig {
            enabled: false,
            enforce_strict: false,
            hsts_enabled: false,
            certificate_validation: false,
            mixed_content_blocking: false,
            allowed_http_domains: vec![
                "localhost".to_string(),
                "127.0.0.1".to_string(),
                "::1".to_string(),
                "0.0.0.0".to_string(),
                "192.168.0.0/16".to_string(),
                "10.0.0.0/8".to_string(),
                "172.16.0.0/12".to_string(),
            ],
            custom_settings: HashMap::new(),
        });

        println!("📋 Loaded {} HTTPS configuration presets", self.presets.len());
    }

    /// Wende Security-Preset an
    pub fn apply_preset(&mut self, preset_name: &str) -> Result<()> {
        if let Some(preset) = self.presets.get(preset_name).cloned() {
            self.config = preset;
            println!("✅ Applied HTTPS preset: {}", preset_name);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Unknown preset: {}", preset_name))
        }
    }

    /// Erstelle Custom-Preset
    pub fn create_custom_preset(&mut self, name: String, config: HttpsConfig) -> Result<()> {
        if self.presets.contains_key(&name) {
            return Err(anyhow::anyhow!("Preset already exists: {}", name));
        }
        
        self.presets.insert(name.clone(), config);
        println!("📝 Created custom preset: {}", name);
        Ok(())
    }

    /// Lösche Custom-Preset
    pub fn delete_preset(&mut self, name: &str) -> Result<()> {
        // Schütze Standard-Presets
        if ["strict", "balanced", "minimal", "development"].contains(&name) {
            return Err(anyhow::anyhow!("Cannot delete built-in preset: {}", name));
        }
        
        if self.presets.remove(name).is_some() {
            println!("🗑️ Deleted preset: {}", name);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Preset not found: {}", name))
        }
    }

    /// Toggle-Funktionen
    pub fn toggle_enabled(&mut self) -> bool {
        self.config.enabled = !self.config.enabled;
        println!("🔒 HTTPS Enforcer: {}", if self.config.enabled { "Enabled" } else { "Disabled" });
        self.config.enabled
    }

    pub fn toggle_strict_mode(&mut self) -> bool {
        self.config.enforce_strict = !self.config.enforce_strict;
        println!("🔒 HTTPS Strict Mode: {}", if self.config.enforce_strict { "Enabled" } else { "Disabled" });
        self.config.enforce_strict
    }

    pub fn toggle_hsts(&mut self) -> bool {
        self.config.hsts_enabled = !self.config.hsts_enabled;
        println!("🔒 HSTS: {}", if self.config.hsts_enabled { "Enabled" } else { "Disabled" });
        self.config.hsts_enabled
    }

    pub fn toggle_certificate_validation(&mut self) -> bool {
        self.config.certificate_validation = !self.config.certificate_validation;
        println!("🔒 Certificate Validation: {}", if self.config.certificate_validation { "Enabled" } else { "Disabled" });
        self.config.certificate_validation
    }

    pub fn toggle_mixed_content_blocking(&mut self) -> bool {
        self.config.mixed_content_blocking = !self.config.mixed_content_blocking;
        println!("🔒 Mixed Content Blocking: {}", if self.config.mixed_content_blocking { "Enabled" } else { "Disabled" });
        self.config.mixed_content_blocking
    }

    /// Exportiere Konfiguration
    pub fn export_configuration(&self) -> Result<String> {
        let export_data = serde_json::json!({
            "config": self.config,
            "custom_presets": self.get_custom_presets(),
            "export_timestamp": chrono::Utc::now().to_rfc3339(),
            "version": "1.0"
        });
        
        let json_data = serde_json::to_string_pretty(&export_data)?;
        println!("📤 Exported HTTPS configuration");
        Ok(json_data)
    }

    /// Importiere Konfiguration
    pub fn import_configuration(&mut self, json_data: &str) -> Result<()> {
        let import_data: serde_json::Value = serde_json::from_str(json_data)?;
        
        // Importiere Haupt-Konfiguration
        if let Some(config_data) = import_data.get("config") {
            self.config = serde_json::from_value(config_data.clone())?;
        }
        
        // Importiere Custom-Presets
        if let Some(presets_data) = import_data.get("custom_presets") {
            if let Ok(custom_presets) = serde_json::from_value::<HashMap<String, HttpsConfig>>(presets_data.clone()) {
                for (name, preset) in custom_presets {
                    if !["strict", "balanced", "minimal", "development"].contains(&name.as_str()) {
                        self.presets.insert(name, preset);
                    }
                }
            }
        }
        
        println!("📥 Imported HTTPS configuration");
        Ok(())
    }

    /// Hole Custom-Presets (ohne Built-ins)
    fn get_custom_presets(&self) -> HashMap<String, HttpsConfig> {
        self.presets.iter()
            .filter(|(name, _)| !["strict", "balanced", "minimal", "development"].contains(&name.as_str()))
            .map(|(name, config)| (name.clone(), config.clone()))
            .collect()
    }

    /// Validiere Konfiguration
    pub fn validate_configuration(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();
        
        // Sicherheits-Validierung
        if self.config.enabled && !self.config.hsts_enabled {
            warnings.push("HSTS ist deaktiviert - empfohlen für bessere Sicherheit".to_string());
        }
        
        if self.config.enabled && !self.config.certificate_validation {
            warnings.push("Certificate Validation ist deaktiviert - Sicherheitsrisiko".to_string());
        }
        
        if self.config.enforce_strict && self.config.allowed_http_domains.len() > 10 {
            warnings.push("Viele HTTP-Ausnahmen im Strict Mode - reduziert Sicherheit".to_string());
        }
        
        // Performance-Validierung
        if self.config.allowed_http_domains.len() > 100 {
            warnings.push("Sehr viele HTTP-Ausnahmen - kann Performance beeinträchtigen".to_string());
        }
        
        // Konsistenz-Validierung
        if !self.config.enabled && (self.config.enforce_strict || self.config.hsts_enabled) {
            warnings.push("HTTPS-Features aktiviert, aber Enforcer deaktiviert".to_string());
        }
        
        Ok(warnings)
    }

    /// Optimiere Konfiguration automatisch
    pub fn optimize_configuration(&mut self) -> Vec<String> {
        let mut optimizations = Vec::new();
        
        // Entferne doppelte Domains
        let initial_count = self.config.allowed_http_domains.len();
        self.config.allowed_http_domains.sort();
        self.config.allowed_http_domains.dedup();
        
        if self.config.allowed_http_domains.len() < initial_count {
            let removed = initial_count - self.config.allowed_http_domains.len();
            optimizations.push(format!("Removed {} duplicate HTTP exception domains", removed));
        }
        
        // Normalisiere Domain-Namen
        for domain in &mut self.config.allowed_http_domains {
            let normalized = domain.to_lowercase().trim().to_string();
            if *domain != normalized {
                *domain = normalized;
                optimizations.push("Normalized domain names to lowercase".to_string());
            }
        }
        
        if optimizations.is_empty() {
            optimizations.push("Configuration already optimized".to_string());
        }
        
        optimizations
    }

    /// Hole Konfiguration-Summary
    pub fn get_config_summary(&self) -> Vec<String> {
        vec![
            "⚙️ HTTPS KONFIGURATION".to_string(),
            "=====================".to_string(),
            "".to_string(),
            format!("🔒 HTTPS Enforcer: {}", if self.config.enabled { "✅ Aktiviert" } else { "❌ Deaktiviert" }),
            format!("🔐 Strict Mode: {}", if self.config.enforce_strict { "✅ Aktiviert" } else { "❌ Deaktiviert" }),
            format!("🛡️ HSTS: {}", if self.config.hsts_enabled { "✅ Aktiviert" } else { "❌ Deaktiviert" }),
            format!("📜 Certificate Validation: {}", if self.config.certificate_validation { "✅ Aktiviert" } else { "❌ Deaktiviert" }),
            format!("🚫 Mixed Content Blocking: {}", if self.config.mixed_content_blocking { "✅ Aktiviert" } else { "❌ Deaktiviert" }),
            "".to_string(),
            format!("🌐 HTTP-Ausnahmen: {} Domains", self.config.allowed_http_domains.len()),
            format!("📋 Verfügbare Presets: {}", self.presets.len()),
            format!("🔧 Custom Settings: {}", self.config.custom_settings.len()),
        ]
    }

    /// Hole verfügbare Presets
    pub fn get_available_presets(&self) -> Vec<String> {
        let mut presets: Vec<_> = self.presets.keys().cloned().collect();
        presets.sort();
        presets
    }

    /// Hole Preset-Details
    pub fn get_preset_details(&self, preset_name: &str) -> Option<String> {
        self.presets.get(preset_name).map(|config| {
            format!(
                "📋 Preset: {}\n\
                 • Enabled: {}\n\
                 • Strict Mode: {}\n\
                 • HSTS: {}\n\
                 • Certificate Validation: {}\n\
                 • Mixed Content Blocking: {}\n\
                 • HTTP Exceptions: {} domains",
                preset_name,
                config.enabled,
                config.enforce_strict,
                config.hsts_enabled,
                config.certificate_validation,
                config.mixed_content_blocking,
                config.allowed_http_domains.len()
            )
        })
    }

    // Getter & Setter
    pub fn get_config(&self) -> &HttpsConfig {
        &self.config
    }

    pub fn get_config_mut(&mut self) -> &mut HttpsConfig {
        &mut self.config
    }

    pub fn set_custom_setting(&mut self, key: String, value: serde_json::Value) {
        self.config.custom_settings.insert(key, value);
    }

    pub fn get_custom_setting(&self, key: &str) -> Option<&serde_json::Value> {
        self.config.custom_settings.get(key)
    }

    pub fn remove_custom_setting(&mut self, key: &str) -> Option<serde_json::Value> {
        self.config.custom_settings.remove(key)
    }
}

impl Default for HttpsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            enforce_strict: false,
            hsts_enabled: true,
            certificate_validation: true,
            mixed_content_blocking: true,
            allowed_http_domains: vec![
                "localhost".to_string(),
                "127.0.0.1".to_string(),
                "::1".to_string(),
            ],
            custom_settings: HashMap::new(),
        }
    }
}

impl Default for HttpsConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_application() {
        let mut manager = HttpsConfigManager::new();
        
        manager.apply_preset("strict").unwrap();
        assert!(manager.get_config().enforce_strict);
        assert!(manager.get_config().hsts_enabled);
        
        manager.apply_preset("minimal").unwrap();
        assert!(!manager.get_config().enforce_strict);
        assert!(!manager.get_config().hsts_enabled);
    }

    #[test]
    fn test_custom_preset_creation() {
        let mut manager = HttpsConfigManager::new();
        let custom_config = HttpsConfig::default();
        
        manager.create_custom_preset("my_preset".to_string(), custom_config).unwrap();
        assert!(manager.get_available_presets().contains(&"my_preset".to_string()));
        
        manager.delete_preset("my_preset").unwrap();
        assert!(!manager.get_available_presets().contains(&"my_preset".to_string()));
    }

    #[test]
    fn test_configuration_export_import() {
        let mut manager1 = HttpsConfigManager::new();
        manager1.toggle_strict_mode();
        
        let exported = manager1.export_configuration().unwrap();
        
        let mut manager2 = HttpsConfigManager::new();
        manager2.import_configuration(&exported).unwrap();
        
        assert_eq!(manager1.get_config().enforce_strict, manager2.get_config().enforce_strict);
    }

    #[test]
    fn test_configuration_validation() {
        let mut manager = HttpsConfigManager::new();
        manager.get_config_mut().hsts_enabled = false;
        
        let warnings = manager.validate_configuration().unwrap();
        assert!(!warnings.is_empty());
        assert!(warnings[0].contains("HSTS"));
    }

    #[test]
    fn test_configuration_optimization() {
        let mut manager = HttpsConfigManager::new();
        manager.get_config_mut().allowed_http_domains.extend(vec![
            "example.com".to_string(),
            "EXAMPLE.COM".to_string(), // Duplicate in different case
            "example.com".to_string(),  // Exact duplicate
        ]);
        
        let optimizations = manager.optimize_configuration();
        assert!(!optimizations.is_empty());
        
        // Should have removed duplicates and normalized case
        let domains = &manager.get_config().allowed_http_domains;
        assert_eq!(domains.iter().filter(|d| d.contains("example.com")).count(), 1);
    }

    #[test]
    fn test_toggle_functions() {
        let mut manager = HttpsConfigManager::new();
        
        let initial_enabled = manager.get_config().enabled;
        let toggled_enabled = manager.toggle_enabled();
        assert_eq!(toggled_enabled, !initial_enabled);
        assert_eq!(manager.get_config().enabled, toggled_enabled);
    }
}