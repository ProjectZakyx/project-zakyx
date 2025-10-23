// 🔧 KONFIGURATIONSVERWALTUNG FÜR ZAKYX BROWSER

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{info, warn};
use crate::error::ZAKYXBrowserError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub primary_port: u16,
    pub fallback_port: u16,
    pub request_timeout_secs: u64,
    pub connect_timeout_secs: u64,
    pub max_redirects: usize,
    pub max_retries: u32,
    pub stack_size_mb: usize,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            primary_port: 3030,
            fallback_port: 3031,
            request_timeout_secs: 30,
            connect_timeout_secs: 10,
            max_redirects: 5,
            max_retries: 3,
            stack_size_mb: 2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub log_level: String,
    pub enable_file_logging: bool,
    pub enable_json_logging: bool,
    pub max_log_files: usize,
    pub log_rotation_days: u64,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            enable_file_logging: true,
            enable_json_logging: true,
            max_log_files: 7,
            log_rotation_days: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enable_cors_bypass: bool,
    pub enable_csp_bypass: bool,
    pub enable_cookie_handling: bool,
    pub enable_https_upgrade: bool,
    pub max_content_size_mb: usize,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_cors_bypass: true,
            enable_csp_bypass: true,
            enable_cookie_handling: true,
            enable_https_upgrade: true,
            max_content_size_mb: 50,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZAKYXConfig {
    pub proxy: ProxyConfig,
    pub logging: LoggingConfig,
    pub security: SecurityConfig,
    pub version: String,
}

impl Default for ZAKYXConfig {
    fn default() -> Self {
        Self {
            proxy: ProxyConfig::default(),
            logging: LoggingConfig::default(),
            security: SecurityConfig::default(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl ZAKYXConfig {
    /// Lädt die Konfiguration aus der Datei oder erstellt eine Standard-Konfiguration
    pub fn load() -> Self {
        let config_path = Self::config_file_path();
        
        match std::fs::read_to_string(&config_path) {
            Ok(content) => {
                match toml::from_str::<ZAKYXConfig>(&content) {
                    Ok(mut config) => {
                        // Version aktualisieren
                        config.version = env!("CARGO_PKG_VERSION").to_string();
                        info!("📁 Configuration loaded from: {:?}", config_path);
                        config
                    }
                    Err(e) => {
                        warn!("⚠️ Failed to parse config file: {}. Using defaults.", e);
                        let default_config = Self::default();
                        let _ = default_config.save();
                        default_config
                    }
                }
            }
            Err(_) => {
                info!("📁 No config file found. Creating default configuration.");
                let default_config = Self::default();
                if let Err(e) = default_config.save() {
                    warn!("⚠️ Failed to save default config: {}", e);
                }
                default_config
            }
        }
    }
    
    /// Speichert die aktuelle Konfiguration in die Datei
    pub fn save(&self) -> Result<(), ZAKYXBrowserError> {
        let config_path = Self::config_file_path();
        
        // Erstelle Verzeichnis falls es nicht existiert
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ZAKYXBrowserError::storage_error("write", &format!("Failed to create config directory: {}", e), Some(&config_path.to_string_lossy())))?;
        }
        
        let content = toml::to_string_pretty(self)
            .map_err(|e| ZAKYXBrowserError::storage_error("write", &format!("Failed to serialize config: {}", e), Some(&config_path.to_string_lossy())))?;
        
        std::fs::write(&config_path, content)
            .map_err(|e| ZAKYXBrowserError::storage_error("write", &format!("Failed to write config file: {}", e), Some(&config_path.to_string_lossy())))?;
        
        info!("💾 Configuration saved to: {:?}", config_path);
        Ok(())
    }
    
    /// Gibt den Pfad zur Konfigurationsdatei zurück
    pub fn config_file_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("zakyx-browser")
            .join("config.toml")
    }
    
    /// Validiert die Konfiguration und korrigiert ungültige Werte
    pub fn validate_and_fix(&mut self) {
        // Proxy-Konfiguration validieren
        if self.proxy.primary_port == 0 {
            warn!("⚠️ Invalid primary port 0, using default 3030");
            self.proxy.primary_port = 3030;
        }
        
        if self.proxy.fallback_port == self.proxy.primary_port {
            warn!("⚠️ Fallback port same as primary, adjusting");
            self.proxy.fallback_port = self.proxy.primary_port + 1;
        }
        
        if self.proxy.request_timeout_secs == 0 {
            warn!("⚠️ Invalid request timeout 0, using default 30s");
            self.proxy.request_timeout_secs = 30;
        }
        
        if self.proxy.stack_size_mb == 0 {
            warn!("⚠️ Invalid stack size 0, using default 2MB");
            self.proxy.stack_size_mb = 2;
        }
        
        // Sicherheits-Konfiguration validieren
        if self.security.max_content_size_mb == 0 {
            warn!("⚠️ Invalid max content size 0, using default 50MB");
            self.security.max_content_size_mb = 50;
        }
        
        // Logging-Konfiguration validieren
        if !["trace", "debug", "info", "warn", "error"].contains(&self.logging.log_level.as_str()) {
            warn!("⚠️ Invalid log level '{}', using 'info'", self.logging.log_level);
            self.logging.log_level = "info".to_string();
        }
        
        info!("✅ Configuration validated and corrected");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = ZAKYXConfig::default();
        assert_eq!(config.proxy.primary_port, 3030);
        assert_eq!(config.proxy.fallback_port, 3031);
        assert_eq!(config.logging.log_level, "info");
        assert!(config.security.enable_cors_bypass);
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = ZAKYXConfig {
            proxy: ProxyConfig {
                primary_port: 0,
                fallback_port: 3030,
                request_timeout_secs: 0,
                stack_size_mb: 0,
                ..Default::default()
            },
            security: SecurityConfig {
                max_content_size_mb: 0,
                ..Default::default()
            },
            logging: LoggingConfig {
                log_level: "invalid".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        
        config.validate_and_fix();
        
        assert_eq!(config.proxy.primary_port, 3030);
        assert_eq!(config.proxy.fallback_port, 3031);
        assert_eq!(config.proxy.request_timeout_secs, 30);
        assert_eq!(config.proxy.stack_size_mb, 2);
        assert_eq!(config.security.max_content_size_mb, 50);
        assert_eq!(config.logging.log_level, "info");
    }
} 
