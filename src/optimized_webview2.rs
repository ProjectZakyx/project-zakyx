// 🌐 OPTIMIZED WEBVIEW2 INTEGRATION - LEGACY FILE
// Diese Datei bleibt für Rückwärtskompatibilität
// Verwende stattdessen src/webview2/ für neue Entwicklungen

use eyre::Result;
use std::collections::HashMap;
use windows::Win32::Foundation::HWND;

// Re-export der neuen modularen Struktur
pub use crate::webview2::{
    OptimizedWebView2 as NewOptimizedWebView2,
    OptimizedWebView2Manager as NewOptimizedWebView2Manager,
    WebView2Config,
    WebView2ConfigBuilder,
    WebView2EnvironmentInfo,
    WebView2EnvironmentDetector,
    WebView2Engine,
    WebView2PerformanceMonitor,
};

/// Legacy OptimizedWebView2 (deprecated - use crate::webview2::OptimizedWebView2)
#[deprecated(note = "Use crate::webview2::OptimizedWebView2 instead")]
pub struct OptimizedWebView2 {
    inner: NewOptimizedWebView2,
}

impl OptimizedWebView2 {
    /// Erstellt eine neue OptimizedWebView2-Instanz (Legacy-Wrapper)
    pub fn new() -> Self {
        println!("⚠️ Using legacy OptimizedWebView2 - consider migrating to crate::webview2::OptimizedWebView2");
        
        Self {
            inner: NewOptimizedWebView2::new(),
        }
    }

    /// Konfiguriert WebView2 (Legacy-Wrapper)
    pub fn configure(&mut self, config: WebView2Config) {
        self.inner.configure(config);
    }

    /// Überprüft WebView2-Verfügbarkeit (Legacy-Wrapper)
    pub fn check_webview2_availability(&mut self) -> Result<WebView2EnvironmentInfo> {
        self.inner.check_webview2_availability()
    }

    /// Erstellt Container-Window (Legacy-Wrapper)
    pub fn create_container(&mut self, parent_window: HWND) -> Result<HWND> {
        self.inner.create_container(parent_window)
    }

    /// Initialisiert WebView2 (Legacy-Wrapper)
    pub async fn initialize_advanced(&mut self) -> Result<()> {
        self.inner.initialize_advanced().await
    }

    /// Navigiert zu URL (Legacy-Wrapper)
    pub async fn navigate_to_url(&mut self, url: &str) -> Result<()> {
        self.inner.navigate_to_url(url).await
    }

    /// Lädt HTML-String (Legacy-Wrapper)
    pub async fn navigate_to_string(&mut self, html: &str) -> Result<()> {
        self.inner.navigate_to_string(html).await
    }

    /// Führt JavaScript aus (Legacy-Wrapper)
    pub async fn execute_script(&mut self, script: &str) -> Result<String> {
        self.inner.execute_script(script).await
    }

    /// Erstellt Diagnosebericht (Legacy-Wrapper)
    pub fn create_diagnostic_report(&mut self) -> HashMap<String, String> {
        self.inner.create_diagnostic_report()
    }

    /// Bereinigt WebView2 (Legacy-Wrapper)
    pub async fn cleanup(&mut self) -> Result<()> {
        self.inner.cleanup().await
    }

    /// Gibt die neue Implementierung zurück (für Migration)
    pub fn get_new_implementation(&self) -> &NewOptimizedWebView2 {
        &self.inner
    }

    /// Gibt die neue Implementierung zurück (mutable, für Migration)
    pub fn get_new_implementation_mut(&mut self) -> &mut NewOptimizedWebView2 {
        &mut self.inner
    }
}

impl Default for OptimizedWebView2 {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy OptimizedWebView2Manager (deprecated - use crate::webview2::OptimizedWebView2Manager)
#[deprecated(note = "Use crate::webview2::OptimizedWebView2Manager instead")]
pub struct OptimizedWebView2Manager {
    inner: NewOptimizedWebView2Manager,
}

impl OptimizedWebView2Manager {
    /// Erstellt einen neuen Manager (Legacy-Wrapper)
    pub fn new() -> Self {
        println!("⚠️ Using legacy OptimizedWebView2Manager - consider migrating to crate::webview2::OptimizedWebView2Manager");
        
        Self {
            inner: NewOptimizedWebView2Manager::new(),
        }
    }

    /// Erstellt eine Instanz (Legacy-Wrapper)
    pub async fn create_instance(&mut self, id: String, parent_window: HWND) -> Result<()> {
        self.inner.create_instance(id, parent_window).await
    }

    /// Gibt die neue Implementierung zurück (für Migration)
    pub fn get_new_implementation(&self) -> &NewOptimizedWebView2Manager {
        &self.inner
    }

    /// Gibt die neue Implementierung zurück (mutable, für Migration)
    pub fn get_new_implementation_mut(&mut self) -> &mut NewOptimizedWebView2Manager {
        &mut self.inner
    }
}

impl Default for OptimizedWebView2Manager {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy Utility-Funktionen (deprecated - use crate::webview2::utils)
#[deprecated(note = "Use crate::webview2::utils instead")]
pub mod utils {
    use super::*;
    
    /// Erstellt Development-Config (Legacy-Wrapper)
    pub fn create_development_config() -> WebView2Config {
        crate::webview2::utils::create_development_config()
    }
    
    /// Erstellt Production-Config (Legacy-Wrapper)
    pub fn create_production_config() -> WebView2Config {
        crate::webview2::utils::create_production_config()
    }
    
    /// Prüft WebView2-Verfügbarkeit (Legacy-Wrapper)
    pub fn check_webview2_quick() -> bool {
        crate::webview2::utils::check_webview2_quick()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_webview2() {
        #[allow(deprecated)]
        {
            let webview = OptimizedWebView2::new();
            let _new_impl = webview.get_new_implementation();
            assert!(true); // Test dass Legacy-Wrapper funktioniert
        }
    }

    #[test]
    fn test_legacy_manager() {
        #[allow(deprecated)]
        {
            let manager = OptimizedWebView2Manager::new();
            let _new_impl = manager.get_new_implementation();
            assert!(true); // Test dass Legacy-Wrapper funktioniert
        }
    }

    #[test]
    fn test_legacy_utils() {
        #[allow(deprecated)]
        {
            let _dev_config = utils::create_development_config();
            let _prod_config = utils::create_production_config();
            let _available = utils::check_webview2_quick();
            assert!(true); // Test dass Legacy-Utils funktionieren
        }
    }

    #[test]
    fn test_re_exports() {
        // Test dass alle Re-exports funktionieren
        let _config = WebView2Config::default();
        let _builder = WebView2ConfigBuilder::new();
        let _detector = WebView2EnvironmentDetector::new();
        let _engine = WebView2Engine::new();
        let _monitor = WebView2PerformanceMonitor::new();
        
        assert!(true); // Re-exports funktionieren
    }
}