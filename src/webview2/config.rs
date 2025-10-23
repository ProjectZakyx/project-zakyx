// 🔧 WebView2 Configuration für ZAKYX Browser
// Konfigurationsmanagement für WebView2-Instanzen

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// WebView2-Konfiguration mit erweiterten Optionen
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebView2Config {
    /// Pfad zum User-Data-Ordner
    pub user_data_folder: String,
    
    /// Zusätzliche Browser-Argumente
    pub additional_browser_arguments: Vec<String>,
    
    /// Single Sign-On erlauben
    pub allow_single_sign_on: bool,
    
    /// Passwort-Autosave aktivieren
    pub enable_password_autosave: bool,
    
    /// Allgemeine Autofill-Funktionen
    pub enable_general_autofill: bool,
    
    /// Pinch-to-Zoom aktivieren
    pub enable_pinch_zoom: bool,
    
    /// Swipe-Navigation aktivieren
    pub enable_swipe_navigation: bool,
    
    /// Browser-Erweiterungen aktivieren
    pub enable_browser_extensions: bool,
    
    /// Standard-Script-Dialoge aktivieren
    pub default_script_dialogs_enabled: bool,
    
    /// Host-Objekte erlauben
    pub host_objects_allowed: bool,
    
    /// Web-Message-API aktivieren
    pub web_message_enabled: bool,
    
    /// Performance-Einstellungen
    pub performance: PerformanceConfig,
    
    /// Sicherheits-Einstellungen
    pub security: SecurityConfig,
    
    /// Debug-Einstellungen
    pub debug: DebugConfig,
}

/// Performance-Konfiguration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PerformanceConfig {
    /// GPU-Rasterisierung aktivieren
    pub enable_gpu_rasterization: bool,
    
    /// Zero-Copy aktivieren
    pub enable_zero_copy: bool,
    
    /// Hardware-Beschleunigung aktivieren
    pub enable_hardware_acceleration: bool,
    
    /// Viz Display Compositor deaktivieren
    pub disable_viz_display_compositor: bool,
    
    /// Memory-Optimierungen
    pub enable_memory_optimization: bool,
    
    /// Cache-Größe in MB
    pub cache_size_mb: u32,
    
    /// Maximale Speichernutzung in MB
    pub max_memory_usage_mb: u32,
}

/// Sicherheits-Konfiguration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityConfig {
    /// Web-Security deaktivieren (nur für Development)
    pub disable_web_security: bool,
    
    /// Unsichere Inhalte erlauben
    pub allow_running_insecure_content: bool,
    
    /// CORS-Prüfungen aktivieren
    pub enable_cors_checks: bool,
    
    /// Mixed-Content blockieren
    pub block_mixed_content: bool,
    
    /// CSP (Content Security Policy) durchsetzen
    pub enforce_csp: bool,
    
    /// Erlaubte Hosts für Cross-Origin-Requests
    pub allowed_hosts: Vec<String>,
}

/// Debug-Konfiguration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DebugConfig {
    /// DevTools aktivieren
    pub enable_dev_tools: bool,
    
    /// Console-Logging aktivieren
    pub enable_console_logging: bool,
    
    /// Performance-Monitoring aktivieren
    pub enable_performance_monitoring: bool,
    
    /// Memory-Profiling aktivieren
    pub enable_memory_profiling: bool,
    
    /// Network-Logging aktivieren
    pub enable_network_logging: bool,
    
    /// Debug-Port (0 = automatisch)
    pub debug_port: u16,
}

impl Default for WebView2Config {
    fn default() -> Self {
        Self {
            user_data_folder: Self::default_user_data_folder(),
            additional_browser_arguments: Self::default_browser_arguments(),
            allow_single_sign_on: false,
            enable_password_autosave: false,
            enable_general_autofill: true,
            enable_pinch_zoom: true,
            enable_swipe_navigation: true,
            enable_browser_extensions: false,
            default_script_dialogs_enabled: true,
            host_objects_allowed: true,
            web_message_enabled: true,
            performance: PerformanceConfig::default(),
            security: SecurityConfig::default(),
            debug: DebugConfig::default(),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_gpu_rasterization: true,
            enable_zero_copy: true,
            enable_hardware_acceleration: true,
            disable_viz_display_compositor: true,
            enable_memory_optimization: true,
            cache_size_mb: 100,
            max_memory_usage_mb: 512,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            disable_web_security: false, // Sicher standardmäßig
            allow_running_insecure_content: false,
            enable_cors_checks: true,
            block_mixed_content: true,
            enforce_csp: true,
            allowed_hosts: Vec::new(),
        }
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            enable_dev_tools: cfg!(debug_assertions),
            enable_console_logging: cfg!(debug_assertions),
            enable_performance_monitoring: false,
            enable_memory_profiling: false,
            enable_network_logging: false,
            debug_port: 0, // Automatisch
        }
    }
}

impl WebView2Config {
    /// Erstellt eine neue Standard-Konfiguration
    pub fn new() -> Self {
        Self::default()
    }

    /// Erstellt eine Development-Konfiguration
    pub fn development() -> Self {
        let mut config = Self::default();
        
        // Performance-Optimierungen für Development
        config.performance.enable_memory_optimization = false;
        config.performance.cache_size_mb = 50;
        
        // Sicherheit für Development lockern
        config.security.disable_web_security = true;
        config.security.allow_running_insecure_content = true;
        config.security.enable_cors_checks = false;
        config.security.block_mixed_content = false;
        config.security.enforce_csp = false;
        
        // Debug-Features aktivieren
        config.debug.enable_dev_tools = true;
        config.debug.enable_console_logging = true;
        config.debug.enable_performance_monitoring = true;
        config.debug.enable_network_logging = true;
        
        config
    }

    /// Erstellt eine Production-Konfiguration
    pub fn production() -> Self {
        let mut config = Self::default();
        
        // Performance-Optimierungen für Production
        config.performance.enable_memory_optimization = true;
        config.performance.cache_size_mb = 200;
        config.performance.max_memory_usage_mb = 1024;
        
        // Sicherheit für Production verschärfen
        config.security.disable_web_security = false;
        config.security.allow_running_insecure_content = false;
        config.security.enable_cors_checks = true;
        config.security.block_mixed_content = true;
        config.security.enforce_csp = true;
        
        // Debug-Features deaktivieren
        config.debug.enable_dev_tools = false;
        config.debug.enable_console_logging = false;
        config.debug.enable_performance_monitoring = false;
        config.debug.enable_network_logging = false;
        
        config
    }

    /// Erstellt eine High-Performance-Konfiguration
    pub fn high_performance() -> Self {
        let mut config = Self::default();
        
        // Maximale Performance-Optimierungen
        config.performance.enable_gpu_rasterization = true;
        config.performance.enable_zero_copy = true;
        config.performance.enable_hardware_acceleration = true;
        config.performance.disable_viz_display_compositor = true;
        config.performance.enable_memory_optimization = true;
        config.performance.cache_size_mb = 500;
        config.performance.max_memory_usage_mb = 2048;
        
        // Performance-orientierte Browser-Argumente
        config.additional_browser_arguments = vec![
            "--enable-gpu-rasterization".to_string(),
            "--enable-zero-copy".to_string(),
            "--disable-features=VizDisplayCompositor".to_string(),
            "--enable-hardware-overlays".to_string(),
            "--enable-gpu-memory-buffer-compositor-resources".to_string(),
            "--enable-gpu-memory-buffer-video-frames".to_string(),
            "--max_old_space_size=4096".to_string(),
        ];
        
        config
    }

    /// Generiert Standard-User-Data-Ordner
    fn default_user_data_folder() -> String {
        std::env::temp_dir()
            .join("ZAKYX")
            .join("WebView2")
            .to_string_lossy()
            .to_string()
    }

    /// Generiert Standard-Browser-Argumente
    fn default_browser_arguments() -> Vec<String> {
        vec![
            "--enable-gpu-rasterization".to_string(),
            "--enable-zero-copy".to_string(),
            "--disable-features=VizDisplayCompositor".to_string(),
        ]
    }

    /// Setzt den User-Data-Ordner
    pub fn with_user_data_folder(mut self, folder: impl Into<String>) -> Self {
        self.user_data_folder = folder.into();
        self
    }

    /// Fügt Browser-Argumente hinzu
    pub fn with_browser_arguments(mut self, args: Vec<String>) -> Self {
        self.additional_browser_arguments.extend(args);
        self
    }

    /// Aktiviert/deaktiviert Single Sign-On
    pub fn with_single_sign_on(mut self, enabled: bool) -> Self {
        self.allow_single_sign_on = enabled;
        self
    }

    /// Aktiviert/deaktiviert Passwort-Autosave
    pub fn with_password_autosave(mut self, enabled: bool) -> Self {
        self.enable_password_autosave = enabled;
        self
    }

    /// Aktiviert/deaktiviert Browser-Erweiterungen
    pub fn with_browser_extensions(mut self, enabled: bool) -> Self {
        self.enable_browser_extensions = enabled;
        self
    }

    /// Setzt Performance-Konfiguration
    pub fn with_performance(mut self, performance: PerformanceConfig) -> Self {
        self.performance = performance;
        self
    }

    /// Setzt Sicherheits-Konfiguration
    pub fn with_security(mut self, security: SecurityConfig) -> Self {
        self.security = security;
        self
    }

    /// Setzt Debug-Konfiguration
    pub fn with_debug(mut self, debug: DebugConfig) -> Self {
        self.debug = debug;
        self
    }

    /// Validiert die Konfiguration
    pub fn validate(&self) -> Result<(), String> {
        // User-Data-Ordner validieren
        if self.user_data_folder.is_empty() {
            return Err("User data folder cannot be empty".to_string());
        }

        // Pfad validieren
        let path = PathBuf::from(&self.user_data_folder);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                return Err(format!("Parent directory does not exist: {}", parent.display()));
            }
        }

        // Performance-Werte validieren
        if self.performance.cache_size_mb == 0 {
            return Err("Cache size must be greater than 0".to_string());
        }

        if self.performance.max_memory_usage_mb < 64 {
            return Err("Max memory usage must be at least 64 MB".to_string());
        }

        // Debug-Port validieren
        if self.debug.debug_port == u16::MAX {
            return Err("Debug port must be <= 65535".to_string());
        }

        Ok(())
    }

    /// Konvertiert zu Browser-Argumenten
    pub fn to_browser_arguments(&self) -> Vec<String> {
        let mut args = self.additional_browser_arguments.clone();

        // Performance-Argumente hinzufügen
        if self.performance.enable_gpu_rasterization {
            args.push("--enable-gpu-rasterization".to_string());
        }

        if self.performance.enable_zero_copy {
            args.push("--enable-zero-copy".to_string());
        }

        if self.performance.disable_viz_display_compositor {
            args.push("--disable-features=VizDisplayCompositor".to_string());
        }

        if self.performance.enable_hardware_acceleration {
            args.push("--enable-hardware-overlays".to_string());
        }

        // Sicherheits-Argumente hinzufügen
        if self.security.disable_web_security {
            args.push("--disable-web-security".to_string());
        }

        if self.security.allow_running_insecure_content {
            args.push("--allow-running-insecure-content".to_string());
        }

        if !self.security.enable_cors_checks {
            args.push("--disable-features=CorsLegacyModeEnabled".to_string());
        }

        // Memory-Argumente hinzufügen
        args.push(format!("--max_old_space_size={}", self.performance.max_memory_usage_mb));

        // Debug-Argumente hinzufügen
        if self.debug.enable_dev_tools {
            args.push("--enable-dev-tools".to_string());
        }

        if self.debug.debug_port > 0 {
            args.push(format!("--remote-debugging-port={}", self.debug.debug_port));
        }

        args
    }

    /// Erstellt User-Data-Ordner falls nicht vorhanden
    pub fn ensure_user_data_folder(&self) -> Result<(), std::io::Error> {
        let path = PathBuf::from(&self.user_data_folder);
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
            println!("📁 Created user data folder: {}", path.display());
        }
        Ok(())
    }

    /// Gibt eine Zusammenfassung der Konfiguration zurück
    pub fn summary(&self) -> String {
        format!(
            "WebView2 Config: {} args, Cache: {}MB, Memory: {}MB, DevTools: {}",
            self.additional_browser_arguments.len(),
            self.performance.cache_size_mb,
            self.performance.max_memory_usage_mb,
            self.debug.enable_dev_tools
        )
    }
}

/// Konfiguration-Builder für fluent API
pub struct WebView2ConfigBuilder {
    config: WebView2Config,
}

impl WebView2ConfigBuilder {
    /// Erstellt einen neuen Builder
    pub fn new() -> Self {
        Self {
            config: WebView2Config::default(),
        }
    }

    /// Setzt den User-Data-Ordner
    pub fn user_data_folder(mut self, folder: impl Into<String>) -> Self {
        self.config.user_data_folder = folder.into();
        self
    }

    /// Fügt Browser-Argumente hinzu
    pub fn add_browser_argument(mut self, arg: impl Into<String>) -> Self {
        self.config.additional_browser_arguments.push(arg.into());
        self
    }

    /// Aktiviert Single Sign-On
    pub fn enable_single_sign_on(mut self) -> Self {
        self.config.allow_single_sign_on = true;
        self
    }

    /// Aktiviert Passwort-Autosave
    pub fn enable_password_autosave(mut self) -> Self {
        self.config.enable_password_autosave = true;
        self
    }

    /// Aktiviert Browser-Erweiterungen
    pub fn enable_browser_extensions(mut self) -> Self {
        self.config.enable_browser_extensions = true;
        self
    }

    /// Setzt Cache-Größe
    pub fn cache_size_mb(mut self, size: u32) -> Self {
        self.config.performance.cache_size_mb = size;
        self
    }

    /// Setzt maximale Speichernutzung
    pub fn max_memory_mb(mut self, memory: u32) -> Self {
        self.config.performance.max_memory_usage_mb = memory;
        self
    }

    /// Aktiviert DevTools
    pub fn enable_dev_tools(mut self) -> Self {
        self.config.debug.enable_dev_tools = true;
        self
    }

    /// Setzt Debug-Port
    pub fn debug_port(mut self, port: u16) -> Self {
        self.config.debug.debug_port = port;
        self
    }

    /// Baut die finale Konfiguration
    pub fn build(self) -> Result<WebView2Config, String> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for WebView2ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = WebView2Config::default();
        assert!(config.validate().is_ok());
        assert!(!config.user_data_folder.is_empty());
        assert!(!config.additional_browser_arguments.is_empty());
    }

    #[test]
    fn test_development_config() {
        let config = WebView2Config::development();
        assert!(config.validate().is_ok());
        assert!(config.security.disable_web_security);
        assert!(config.debug.enable_dev_tools);
    }

    #[test]
    fn test_production_config() {
        let config = WebView2Config::production();
        assert!(config.validate().is_ok());
        assert!(!config.security.disable_web_security);
        assert!(!config.debug.enable_dev_tools);
    }

    #[test]
    fn test_high_performance_config() {
        let config = WebView2Config::high_performance();
        assert!(config.validate().is_ok());
        assert!(config.performance.enable_gpu_rasterization);
        assert!(config.performance.cache_size_mb >= 500);
    }

    #[test]
    fn test_config_builder() {
        let config = WebView2ConfigBuilder::new()
            .user_data_folder("test_folder")
            .cache_size_mb(200)
            .enable_dev_tools()
            .debug_port(9222)
            .build();

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.user_data_folder, "test_folder");
        assert_eq!(config.performance.cache_size_mb, 200);
        assert!(config.debug.enable_dev_tools);
        assert_eq!(config.debug.debug_port, 9222);
    }

    #[test]
    fn test_config_validation() {
        let mut config = WebView2Config::default();
        
        // Leerer User-Data-Ordner sollte fehlschlagen
        config.user_data_folder = String::new();
        assert!(config.validate().is_err());

        // Cache-Größe 0 sollte fehlschlagen
        config.user_data_folder = "test".to_string();
        config.performance.cache_size_mb = 0;
        assert!(config.validate().is_err());

        // Zu wenig Memory sollte fehlschlagen
        config.performance.cache_size_mb = 100;
        config.performance.max_memory_usage_mb = 32;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_browser_arguments_generation() {
        let config = WebView2Config::development();
        let args = config.to_browser_arguments();
        
        assert!(args.contains(&"--disable-web-security".to_string()));
        assert!(args.contains(&"--allow-running-insecure-content".to_string()));
        assert!(args.iter().any(|arg| arg.starts_with("--max_old_space_size=")));
    }

    #[test]
    fn test_config_summary() {
        let config = WebView2Config::default();
        let summary = config.summary();
        
        assert!(summary.contains("WebView2 Config"));
        assert!(summary.contains("Cache"));
        assert!(summary.contains("Memory"));
        assert!(summary.contains("DevTools"));
    }

    #[test]
    fn test_fluent_api() {
        let config = WebView2Config::new()
            .with_user_data_folder("custom_folder")
            .with_single_sign_on(true)
            .with_browser_extensions(true);

        assert_eq!(config.user_data_folder, "custom_folder");
        assert!(config.allow_single_sign_on);
        assert!(config.enable_browser_extensions);
    }
}