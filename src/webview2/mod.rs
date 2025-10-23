// 🌐 WebView2 Module für ZAKYX Browser
// Modulare WebView2-Integration mit erweiterten Features

#![allow(dead_code)] // WebView2 system API - comprehensive API kept for extensibility

pub mod config;
pub mod environment;
pub mod engine;
pub mod performance;
pub mod manager;

// Re-exports für einfache Verwendung
pub use config::{
    WebView2Config,
    WebView2ConfigBuilder,
    PerformanceConfig,
    SecurityConfig,
    DebugConfig,
};

pub use environment::{
    WebView2EnvironmentDetector,
    WebView2EnvironmentInfo,
    WebView2RuntimeType,
    WebView2Channel,
};

pub use engine::{
    WebView2Engine,
    NavigationEntry,
    NavigationType,
    JavaScriptContext,
    JavaScriptExecution,
};

pub use performance::{
    WebView2PerformanceMonitor,
    PerformanceMetrics,
    NavigationMetrics,
    JavaScriptMetrics,
    MemoryMetrics,
    RenderingMetrics,
    NetworkMetrics,
    SystemMetrics,
    PerformanceWarning,
    OptimizationSuggestion,
    WarningType,
    WarningSeverity,
    OptimizationCategory,
};

pub use manager::{
    OptimizedWebView2Manager,
    WebView2Instance,
    InstanceStatus,
    InstanceMetrics,
    ManagerStatistics,
};

/// Hauptklasse für WebView2-Integration
/// 
/// Diese Klasse bietet eine einheitliche Schnittstelle für alle WebView2-Funktionen
/// und abstrahiert die Komplexität der verschiedenen Module.
/// 
/// # Beispiel
/// 
/// ```rust
/// use crate::webview2::OptimizedWebView2;
/// 
/// let mut webview = OptimizedWebView2::new();
/// webview.initialize().await?;
/// webview.navigate_to_url("https://example.com").await?;
/// ```
#[derive(Debug)]
pub struct OptimizedWebView2 {
    /// WebView2-Manager
    manager: OptimizedWebView2Manager,
    
    /// Standard-Instanz-ID
    default_instance_id: String,
    
    /// Konfiguration
    config: WebView2Config,
    
    /// Environment-Informationen
    environment_info: Option<WebView2EnvironmentInfo>,
    
    /// Initialisierungsstatus
    is_initialized: bool,
    
    /// Initialisierungsversuche
    initialization_attempts: u32,
    
    /// Letzter Fehler
    last_error: Option<String>,
    
    /// Container-Window (für Legacy-Kompatibilität)
    container_window: Option<windows::Win32::Foundation::HWND>,
}

impl OptimizedWebView2 {
    /// Erstellt eine neue OptimizedWebView2-Instanz
    pub fn new() -> Self {
        println!("🌐 Creating OptimizedWebView2...");
        
        Self {
            manager: OptimizedWebView2Manager::new(),
            default_instance_id: "default".to_string(),
            config: WebView2Config::default(),
            environment_info: None,
            is_initialized: false,
            initialization_attempts: 0,
            last_error: None,
            container_window: None,
        }
    }

    /// Erstellt eine OptimizedWebView2 mit spezifischer Konfiguration
    pub fn with_config(config: WebView2Config) -> Self {
        println!("🌐 Creating OptimizedWebView2 with custom config...");
        
        let mut webview = Self::new();
        webview.config = config.clone();
        webview.manager = OptimizedWebView2Manager::with_global_config(config);
        webview
    }

    /// Konfiguriert die WebView2-Instanz
    pub fn configure(&mut self, config: WebView2Config) {
        println!("⚙️ Configuring OptimizedWebView2...");
        self.config = config.clone();
        self.manager = OptimizedWebView2Manager::with_global_config(config);
        println!("✅ OptimizedWebView2 configuration updated!");
    }

    /// Überprüft WebView2-Verfügbarkeit
    pub fn check_webview2_availability(&mut self) -> anyhow::Result<WebView2EnvironmentInfo> {
        println!("🔍 Checking WebView2 availability...");
        
        let mut detector = WebView2EnvironmentDetector::new();
        let info = detector.check_availability()?;
        
        println!("📋 WebView2 Environment Info:");
        println!("   Available: {}", info.is_available);
        println!("   Version: {}", info.version);
        println!("   Path: {}", info.installation_path);
        println!("   Runtime Type: {}", info.runtime_type);
        
        if !info.is_available {
            let error = "WebView2 Runtime not available";
            self.last_error = Some(error.to_string());
            return Err(anyhow::anyhow!(error));
        }
        
        self.environment_info = Some(info.clone());
        Ok(info)
    }

    /// Erstellt das Container-Window (für Legacy-Kompatibilität)
    pub fn create_container(&mut self, parent_window: windows::Win32::Foundation::HWND) -> anyhow::Result<windows::Win32::Foundation::HWND> {
        println!("🏗️ Creating WebView2 container window...");
        
        self.container_window = Some(parent_window);
        
        // Container wird vom Manager/Engine erstellt
        println!("✅ WebView2 container prepared: {:?}", parent_window);
        Ok(parent_window)
    }

    /// Initialisiert WebView2 mit erweiterten Optionen
    pub async fn initialize_advanced(&mut self) -> anyhow::Result<()> {
        println!("🚀 Initializing Advanced WebView2...");
        self.initialization_attempts += 1;
        
        // Manager initialisieren
        self.manager.initialize().await?;
        
        // Standard-Instanz erstellen
        if let Some(parent_window) = self.container_window {
            self.manager.create_instance_with_config(
                self.default_instance_id.clone(),
                parent_window,
                Some(self.config.clone())
            ).await?;
        } else {
            // Fallback: Null-Window verwenden
            let null_window = windows::Win32::Foundation::HWND(std::ptr::null_mut());
            self.manager.create_instance_with_config(
                self.default_instance_id.clone(),
                null_window,
                Some(self.config.clone())
            ).await?;
        }
        
        self.is_initialized = true;
        println!("✅ Advanced WebView2 initialized successfully!");
        
        Ok(())
    }

    /// Navigiert zu einer URL
    pub async fn navigate_to_url(&mut self, url: &str) -> anyhow::Result<()> {
        if !self.is_initialized {
            return Err(anyhow::anyhow!("WebView2 not initialized"));
        }
        
        self.manager.navigate_instance_to_url(&self.default_instance_id, url).await?;
        Ok(())
    }

    /// Lädt HTML-String
    pub async fn navigate_to_string(&mut self, html: &str) -> anyhow::Result<()> {
        if !self.is_initialized {
            return Err(anyhow::anyhow!("WebView2 not initialized"));
        }
        
        // HTML als Data-URL laden
        let data_url = format!("data:text/html,{}", urlencoding::encode(html));
        self.manager.navigate_instance_to_url(&self.default_instance_id, &data_url).await?;
        Ok(())
    }

    /// Führt JavaScript aus
    pub async fn execute_script(&mut self, script: &str) -> anyhow::Result<String> {
        if !self.is_initialized {
            return Err(anyhow::anyhow!("WebView2 not initialized"));
        }
        
        let result = self.manager.execute_script_in_instance(&self.default_instance_id, script).await?;
        Ok(result)
    }

    /// Erstellt einen Diagnosebericht
    pub fn create_diagnostic_report(&mut self) -> std::collections::HashMap<String, String> {
        let mut report = std::collections::HashMap::new();
        
        // Basis-Informationen
        report.insert("initialized".to_string(), self.is_initialized.to_string());
        report.insert("initialization_attempts".to_string(), self.initialization_attempts.to_string());
        report.insert("manager_initialized".to_string(), self.manager.is_initialized().to_string());
        report.insert("instance_count".to_string(), self.manager.instance_count().to_string());
        
        // Letzter Fehler
        if let Some(error) = &self.last_error {
            report.insert("last_error".to_string(), error.clone());
        }
        
        // Environment-Informationen
        if let Some(env_info) = &self.environment_info {
            report.insert("webview2_version".to_string(), env_info.version.clone());
            report.insert("webview2_available".to_string(), env_info.is_available.to_string());
            report.insert("runtime_type".to_string(), env_info.runtime_type.to_string());
        }
        
        // Konfiguration
        report.insert("config_summary".to_string(), self.config.summary());
        report.insert("user_data_folder".to_string(), self.config.user_data_folder.clone());
        report.insert("browser_args_count".to_string(), self.config.additional_browser_arguments.len().to_string());
        
        // Manager-Statistiken
        let stats = self.manager.get_statistics();
        report.insert("total_navigations".to_string(), stats.total_navigations.to_string());
        report.insert("total_js_executions".to_string(), stats.total_javascript_executions.to_string());
        
        // Container-Window
        if let Some(container) = self.container_window {
            report.insert("container_window".to_string(), format!("{:?}", container));
        }
        
        report
    }

    /// Bereinigt die WebView2-Instanz
    pub async fn cleanup(&mut self) -> anyhow::Result<()> {
        println!("🧹 Cleaning up OptimizedWebView2...");
        
        // Manager bereinigen
        self.manager.cleanup().await?;
        
        // Status zurücksetzen
        self.is_initialized = false;
        self.container_window = None;
        self.environment_info = None;
        self.last_error = None;
        
        println!("✅ OptimizedWebView2 cleaned up!");
        Ok(())
    }

    /// Gibt den Manager zurück (für erweiterte Funktionen)
    pub fn get_manager(&self) -> &OptimizedWebView2Manager {
        &self.manager
    }

    /// Gibt den Manager zurück (mutable, für erweiterte Funktionen)
    pub fn get_manager_mut(&mut self) -> &mut OptimizedWebView2Manager {
        &mut self.manager
    }

    /// Gibt die Standard-Instanz zurück
    pub fn get_default_instance(&self) -> Option<&WebView2Instance> {
        self.manager.get_instance(&self.default_instance_id)
    }

    /// Gibt die Standard-Instanz zurück (mutable)
    pub fn get_default_instance_mut(&mut self) -> Option<&mut WebView2Instance> {
        self.manager.get_instance_mut(&self.default_instance_id)
    }

    /// Aktualisiert Performance-Metriken
    pub fn update_performance(&mut self) {
        self.manager.update_performance();
    }

    /// Gibt einen Performance-Report zurück
    pub fn get_performance_report(&self) -> String {
        self.manager.generate_manager_report()
    }

    // Getter
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    pub fn get_config(&self) -> &WebView2Config {
        &self.config
    }

    pub fn get_environment_info(&self) -> Option<&WebView2EnvironmentInfo> {
        self.environment_info.as_ref()
    }

    pub fn get_container_window(&self) -> Option<windows::Win32::Foundation::HWND> {
        self.container_window
    }

    pub fn get_last_error(&self) -> Option<&String> {
        self.last_error.as_ref()
    }

    pub fn get_initialization_attempts(&self) -> u32 {
        self.initialization_attempts
    }
}

impl Default for OptimizedWebView2 {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility-Funktionen für WebView2
pub mod utils {
    use super::*;
    
    /// Erstellt eine Development-Konfiguration
    pub fn create_development_config() -> WebView2Config {
        WebView2Config::development()
    }
    
    /// Erstellt eine Production-Konfiguration
    pub fn create_production_config() -> WebView2Config {
        WebView2Config::production()
    }
    
    /// Erstellt eine High-Performance-Konfiguration
    pub fn create_high_performance_config() -> WebView2Config {
        WebView2Config::high_performance()
    }
    
    /// Prüft WebView2-Verfügbarkeit schnell
    pub fn check_webview2_quick() -> bool {
        let mut detector = WebView2EnvironmentDetector::new();
        detector.check_availability()
            .map(|info| info.is_available)
            .unwrap_or(false)
    }
    
    /// Erstellt einen minimalen WebView2 für Tests
    pub fn create_test_webview() -> OptimizedWebView2 {
        let config = WebView2ConfigBuilder::new()
            .enable_dev_tools()
            .cache_size_mb(50)
            .debug_port(9222)
            .build()
            .unwrap_or_else(|_| WebView2Config::development());
            
        OptimizedWebView2::with_config(config)
    }
    
    /// Gibt eine Environment-Diagnose zurück
    pub fn get_environment_diagnosis() -> anyhow::Result<String> {
        let mut detector = WebView2EnvironmentDetector::new();
        detector.get_diagnostic_report()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_webview2_creation() {
        let webview = OptimizedWebView2::new();
        assert!(!webview.is_initialized);
        assert_eq!(webview.initialization_attempts, 0);
        assert!(webview.container_window.is_none());
    }

    #[test]
    fn test_webview2_with_config() {
        let config = WebView2Config::development();
        let webview = OptimizedWebView2::with_config(config.clone());
        assert_eq!(webview.config.debug.enable_dev_tools, config.debug.enable_dev_tools);
    }

    #[test]
    fn test_diagnostic_report() {
        let mut webview = OptimizedWebView2::new();
        let report = webview.create_diagnostic_report();
        
        assert!(report.contains_key("initialized"));
        assert!(report.contains_key("initialization_attempts"));
        assert!(report.contains_key("config_summary"));
    }

    #[test]
    fn test_utils_functions() {
        let dev_config = utils::create_development_config();
        assert!(dev_config.debug.enable_dev_tools);
        
        let prod_config = utils::create_production_config();
        assert!(!prod_config.debug.enable_dev_tools);
        
        let perf_config = utils::create_high_performance_config();
        assert!(perf_config.performance.cache_size_mb >= 500);
        
        let test_webview = utils::create_test_webview();
        assert!(test_webview.config.debug.enable_dev_tools);
    }

    #[test]
    fn test_webview2_configuration() {
        let mut webview = OptimizedWebView2::new();
        let config = WebView2Config::high_performance();
        
        webview.configure(config.clone());
        assert_eq!(webview.config.performance.cache_size_mb, config.performance.cache_size_mb);
    }

    #[test]
    fn test_container_creation() {
        let mut webview = OptimizedWebView2::new();
        let hwnd = windows::Win32::Foundation::HWND(std::ptr::null_mut());
        
        let result = webview.create_container(hwnd);
        assert!(result.is_ok());
        assert_eq!(webview.container_window, Some(hwnd));
    }

    #[tokio::test]
    async fn test_webview2_cleanup() {
        let mut webview = OptimizedWebView2::new();
        let result = webview.cleanup().await;
        
        assert!(result.is_ok());
        assert!(!webview.is_initialized);
        assert!(webview.container_window.is_none());
    }

    #[test]
    fn test_manager_access() {
        let webview = OptimizedWebView2::new();
        let manager = webview.get_manager();
        
        assert!(!manager.is_initialized());
        assert_eq!(manager.instance_count(), 0);
    }
}
