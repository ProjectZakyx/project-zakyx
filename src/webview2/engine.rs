// 🚀 WebView2 Core Engine für ZAKYX Browser
// Kernfunktionalität für WebView2-Integration

use anyhow::Result;
use std::collections::HashMap;
use windows::Win32::{
    Foundation::*,
    UI::WindowsAndMessaging::*,
    System::Com::*,
};
use crate::webview2::{config::WebView2Config, environment::{WebView2EnvironmentDetector, WebView2EnvironmentInfo}};
// Windows API macros
macro_rules! w {
    ($s:literal) => {
        windows::core::PCWSTR::from_raw(concat!($s, "\0").encode_utf16().collect::<Vec<u16>>().as_ptr())
    };
}

/// WebView2-Core-Engine
#[derive(Debug)]
pub struct WebView2Engine {
    /// Konfiguration
    config: WebView2Config,
    
    /// Environment-Informationen
    environment_info: Option<WebView2EnvironmentInfo>,
    
    /// Container-Window
    container_window: Option<HWND>,
    
    /// Initialisierungsstatus
    is_initialized: bool,
    
    /// Initialisierungsversuche
    initialization_attempts: u32,
    
    /// Letzter Fehler
    last_error: Option<String>,
    
    /// Environment-Detector
    environment_detector: WebView2EnvironmentDetector,
    
    /// Navigation-Historie
    navigation_history: Vec<NavigationEntry>,
    
    /// Aktuelle URL
    current_url: Option<String>,
    
    /// Loading-Status
    is_loading: bool,
    
    /// JavaScript-Context
    js_context: JavaScriptContext,
}

/// Navigation-Eintrag
#[derive(Debug, Clone)]
pub struct NavigationEntry {
    /// URL
    pub url: String,
    
    /// Titel
    pub title: Option<String>,
    
    /// Zeitstempel
    pub timestamp: std::time::SystemTime,
    
    /// Navigation-Typ
    pub navigation_type: NavigationType,
    
    /// Erfolgreich
    pub success: bool,
    
    /// Fehler (falls vorhanden)
    pub error: Option<String>,
}

/// Navigation-Typ
#[derive(Debug, Clone, PartialEq)]
pub enum NavigationType {
    /// Normale Navigation
    Navigate,
    
    /// Reload
    Reload,
    
    /// Zurück
    Back,
    
    /// Vorwärts
    Forward,
    
    /// HTML-String laden
    LoadString,
    
    /// JavaScript-Navigation
    JavaScript,
}

/// JavaScript-Context
#[derive(Debug)]
pub struct JavaScriptContext {
    /// Ausgeführte Scripts
    executed_scripts: Vec<JavaScriptExecution>,
    
    /// Injizierte Scripts
    injected_scripts: HashMap<String, String>,
    
    /// Event-Listener
    event_listeners: HashMap<String, Vec<String>>,
}

/// JavaScript-Ausführung
#[derive(Debug, Clone)]
pub struct JavaScriptExecution {
    /// Script-Code
    pub script: String,
    
    /// Ergebnis
    pub result: Option<String>,
    
    /// Fehler
    pub error: Option<String>,
    
    /// Zeitstempel
    pub timestamp: std::time::SystemTime,
    
    /// Ausführungszeit in Millisekunden
    pub execution_time_ms: u64,
}

impl WebView2Engine {
    /// Erstellt eine neue WebView2-Engine
    pub fn new() -> Self {
        println!("🚀 Creating WebView2 Engine...");
        
        Self {
            config: WebView2Config::default(),
            environment_info: None,
            container_window: None,
            is_initialized: false,
            initialization_attempts: 0,
            last_error: None,
            environment_detector: WebView2EnvironmentDetector::new(),
            navigation_history: Vec::new(),
            current_url: None,
            is_loading: false,
            js_context: JavaScriptContext::new(),
        }
    }

    /// Erstellt eine Engine mit spezifischer Konfiguration
    pub fn with_config(config: WebView2Config) -> Self {
        println!("🚀 Creating WebView2 Engine with custom config...");
        
        let mut engine = Self::new();
        engine.config = config;
        engine
    }

    /// Konfiguriert die Engine
    pub fn configure(&mut self, config: WebView2Config) -> Result<()> {
        println!("⚙️ Configuring WebView2 Engine...");
        
        // Validiere Konfiguration
        config.validate().map_err(|e| anyhow::anyhow!("Invalid configuration: {}", e))?;
        
        // User-Data-Ordner erstellen
        config.ensure_user_data_folder()
            .map_err(|e| anyhow::anyhow!("Failed to create user data folder: {}", e))?;
        
        self.config = config;
        println!("✅ WebView2 Engine configured: {}", self.config.summary());
        
        Ok(())
    }

    /// Überprüft WebView2-Verfügbarkeit
    pub fn check_environment(&mut self) -> Result<WebView2EnvironmentInfo> {
        println!("🔍 Checking WebView2 environment...");
        
        let info = self.environment_detector.check_availability()?;
        
        println!("📋 WebView2 Environment Info:");
        println!("   Available: {}", info.is_available);
        println!("   Version: {}", info.version);
        println!("   Path: {}", info.installation_path);
        println!("   Runtime Type: {}", info.runtime_type);
        println!("   Channel: {}", info.channel);
        
        if !info.is_available {
            let error = "WebView2 Runtime not available. Please install Microsoft Edge WebView2.";
            self.last_error = Some(error.to_string());
            return Err(anyhow::anyhow!(error));
        }
        
        self.environment_info = Some(info.clone());
        Ok(info)
    }

    /// Erstellt das Container-Window
    pub fn create_container(&mut self, parent_window: HWND) -> Result<HWND> {
        println!("🏗️ Creating WebView2 container window...");
        
        unsafe {
            let container = CreateWindowExW(
                WS_EX_CONTROLPARENT,
                w!("STATIC"),
                w!("ZAKYX_WebView2_Container"),
                WS_CHILD | WS_VISIBLE | WS_CLIPCHILDREN | WS_CLIPSIBLINGS,
                0, 0, 800, 600,
                Some(parent_window),
                None,
                None,
                None,
            );
            
            match container {
                Ok(hwnd) => {
                    self.container_window = Some(hwnd);
                    println!("✅ WebView2 container created: {:?}", hwnd);
                    Ok(hwnd)
                }
                Err(_) => {
                    let error = "Failed to create WebView2 container window";
                    self.last_error = Some(error.to_string());
                    Err(anyhow::anyhow!(error))
                }
            }
        }
    }

    /// Initialisiert die WebView2-Engine
    pub async fn initialize(&mut self) -> Result<()> {
        println!("🚀 Initializing WebView2 Engine...");
        self.initialization_attempts += 1;
        
        // Environment prüfen
        self.check_environment()?;
        
        // COM initialisieren
        self.initialize_com()?;
        
        // WebView2 initialisieren (simuliert)
        self.initialize_webview2().await?;
        
        self.is_initialized = true;
        println!("✅ WebView2 Engine initialized successfully!");
        
        Ok(())
    }

    /// Initialisiert COM
    fn initialize_com(&self) -> Result<()> {
        println!("🔧 Initializing COM...");
        
        unsafe {
            let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            if hr.is_err() && hr.0 != 0x80010106u32 as i32 { // RPC_E_CHANGED_MODE
                return Err(anyhow::anyhow!("Failed to initialize COM: {:?}", hr));
            }
        }
        
        println!("✅ COM initialized");
        Ok(())
    }

    /// Initialisiert WebView2 (simuliert)
    async fn initialize_webview2(&mut self) -> Result<()> {
        println!("🌐 Initializing WebView2 runtime...");
        
        // Simuliere WebView2-Initialisierung
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Konfiguration anwenden
        self.apply_configuration().await?;
        
        println!("✅ WebView2 runtime initialized");
        Ok(())
    }

    /// Wendet die Konfiguration an
    async fn apply_configuration(&self) -> Result<()> {
        println!("⚙️ Applying WebView2 configuration...");
        
        // Browser-Argumente anwenden (simuliert)
        let args = self.config.to_browser_arguments();
        println!("📋 Browser arguments: {:?}", args);
        
        // Simuliere Konfigurationsanwendung
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        
        println!("✅ Configuration applied");
        Ok(())
    }

    /// Navigiert zu einer URL
    pub async fn navigate_to_url(&mut self, url: &str) -> Result<()> {
        if !self.is_initialized {
            return Err(anyhow::anyhow!("WebView2 Engine not initialized"));
        }
        
        println!("🌐 Navigating to: {}", url);
        
        // URL-Validierung
        if !self.is_valid_url(url) {
            let error = format!("Invalid URL format: {}", url);
            self.add_navigation_entry(url, NavigationType::Navigate, false, Some(error.clone()));
            return Err(anyhow::anyhow!(error));
        }
        
        self.is_loading = true;
        let start_time = std::time::Instant::now();
        
        // Navigation ausführen (simuliert)
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        let elapsed = start_time.elapsed();
        self.is_loading = false;
        self.current_url = Some(url.to_string());
        
        // Navigation-Eintrag hinzufügen
        self.add_navigation_entry(url, NavigationType::Navigate, true, None);
        
        println!("✅ Navigation completed in {}ms", elapsed.as_millis());
        Ok(())
    }

    /// Lädt HTML-String
    pub async fn navigate_to_string(&mut self, html: &str) -> Result<()> {
        if !self.is_initialized {
            return Err(anyhow::anyhow!("WebView2 Engine not initialized"));
        }
        
        println!("📄 Loading HTML string ({} chars)...", html.len());
        
        // HTML-Validierung
        if html.trim().is_empty() {
            let error = "Empty HTML content";
            self.add_navigation_entry("data:text/html", NavigationType::LoadString, false, Some(error.to_string()));
            return Err(anyhow::anyhow!(error));
        }
        
        self.is_loading = true;
        let start_time = std::time::Instant::now();
        
        // HTML laden (simuliert)
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        
        let elapsed = start_time.elapsed();
        self.is_loading = false;
        self.current_url = Some("data:text/html".to_string());
        
        // Navigation-Eintrag hinzufügen
        self.add_navigation_entry("data:text/html", NavigationType::LoadString, true, None);
        
        println!("✅ HTML string loaded in {}ms", elapsed.as_millis());
        Ok(())
    }

    /// Führt JavaScript aus
    pub async fn execute_script(&mut self, script: &str) -> Result<String> {
        if !self.is_initialized {
            return Err(anyhow::anyhow!("WebView2 Engine not initialized"));
        }
        
        println!("💉 Executing JavaScript ({} chars)...", script.len());
        
        // Script-Validierung
        if script.trim().is_empty() {
            return Err(anyhow::anyhow!("Empty script"));
        }
        
        let start_time = std::time::Instant::now();
        
        // Script ausführen (simuliert)
        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
        
        let elapsed = start_time.elapsed();
        let result = format!("Script executed successfully ({}ms)", elapsed.as_millis());
        
        // JavaScript-Ausführung protokollieren
        self.js_context.add_execution(JavaScriptExecution {
            script: script.to_string(),
            result: Some(result.clone()),
            error: None,
            timestamp: std::time::SystemTime::now(),
            execution_time_ms: elapsed.as_millis() as u64,
        });
        
        println!("✅ JavaScript executed: {}", result);
        Ok(result)
    }

    /// Geht zurück in der Navigation
    pub async fn go_back(&mut self) -> Result<()> {
        if !self.is_initialized {
            return Err(anyhow::anyhow!("WebView2 Engine not initialized"));
        }
        
        println!("⬅️ Going back...");
        
        // Simuliere Zurück-Navigation
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        
        self.add_navigation_entry("back", NavigationType::Back, true, None);
        
        println!("✅ Went back");
        Ok(())
    }

    /// Geht vorwärts in der Navigation
    pub async fn go_forward(&mut self) -> Result<()> {
        if !self.is_initialized {
            return Err(anyhow::anyhow!("WebView2 Engine not initialized"));
        }
        
        println!("➡️ Going forward...");
        
        // Simuliere Vorwärts-Navigation
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        
        self.add_navigation_entry("forward", NavigationType::Forward, true, None);
        
        println!("✅ Went forward");
        Ok(())
    }

    /// Lädt die Seite neu
    pub async fn reload(&mut self) -> Result<()> {
        if !self.is_initialized {
            return Err(anyhow::anyhow!("WebView2 Engine not initialized"));
        }
        
        println!("🔄 Reloading...");
        
        self.is_loading = true;
        
        // Simuliere Reload
        tokio::time::sleep(tokio::time::Duration::from_millis(80)).await;
        
        self.is_loading = false;
        
        let current_url = self.current_url.clone().unwrap_or_else(|| "unknown".to_string());
        self.add_navigation_entry(&current_url, NavigationType::Reload, true, None);
        
        println!("✅ Reloaded");
        Ok(())
    }

    /// Stoppt das Laden
    pub fn stop_loading(&mut self) -> Result<()> {
        if !self.is_initialized {
            return Err(anyhow::anyhow!("WebView2 Engine not initialized"));
        }
        
        println!("⏹️ Stopping loading...");
        
        self.is_loading = false;
        
        println!("✅ Loading stopped");
        Ok(())
    }

    /// Validiert eine URL
    fn is_valid_url(&self, url: &str) -> bool {
        url.starts_with("http://") || 
        url.starts_with("https://") || 
        url.starts_with("file://") ||
        url.starts_with("data:") ||
        url.starts_with("about:")
    }

    /// Fügt einen Navigation-Eintrag hinzu
    fn add_navigation_entry(&mut self, url: &str, nav_type: NavigationType, success: bool, error: Option<String>) {
        let entry = NavigationEntry {
            url: url.to_string(),
            title: None,
            timestamp: std::time::SystemTime::now(),
            navigation_type: nav_type,
            success,
            error,
        };
        
        self.navigation_history.push(entry);
        
        // Historie begrenzen (letzte 100 Einträge)
        if self.navigation_history.len() > 100 {
            self.navigation_history.remove(0);
        }
    }

    /// Gibt die Navigation-Historie zurück
    pub fn get_navigation_history(&self) -> &[NavigationEntry] {
        &self.navigation_history
    }

    /// Gibt die aktuelle URL zurück
    pub fn get_current_url(&self) -> Option<&String> {
        self.current_url.as_ref()
    }

    /// Prüft ob gerade geladen wird
    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    /// Gibt die JavaScript-Historie zurück
    pub fn get_javascript_history(&self) -> &[JavaScriptExecution] {
        &self.js_context.executed_scripts
    }

    /// Erstellt einen Diagnosebericht
    pub fn create_diagnostic_report(&mut self) -> Result<HashMap<String, String>> {
        let mut report = HashMap::new();
        
        // Basis-Informationen
        report.insert("initialized".to_string(), self.is_initialized.to_string());
        report.insert("initialization_attempts".to_string(), self.initialization_attempts.to_string());
        report.insert("is_loading".to_string(), self.is_loading.to_string());
        
        // Aktuelle URL
        if let Some(url) = &self.current_url {
            report.insert("current_url".to_string(), url.clone());
        }
        
        // Fehler
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
        
        // Statistiken
        report.insert("navigation_history_count".to_string(), self.navigation_history.len().to_string());
        report.insert("javascript_executions".to_string(), self.js_context.executed_scripts.len().to_string());
        
        // Container-Window
        if let Some(container) = self.container_window {
            report.insert("container_window".to_string(), format!("{:?}", container));
        }
        
        Ok(report)
    }

    /// Bereinigt die Engine
    pub fn cleanup(&mut self) -> Result<()> {
        println!("🧹 Cleaning up WebView2 Engine...");
        
        // Container-Window zerstören
        if let Some(container) = self.container_window {
            unsafe {
                let _ = DestroyWindow(container);
            }
            self.container_window = None;
        }
        
        // Status zurücksetzen
        self.is_initialized = false;
        self.is_loading = false;
        self.current_url = None;
        
        // COM bereinigen
        unsafe {
            CoUninitialize();
        }
        
        println!("✅ WebView2 Engine cleaned up");
        Ok(())
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

    pub fn get_container_window(&self) -> Option<HWND> {
        self.container_window
    }

    pub fn get_last_error(&self) -> Option<&String> {
        self.last_error.as_ref()
    }

    pub fn get_initialization_attempts(&self) -> u32 {
        self.initialization_attempts
    }
}

impl JavaScriptContext {
    /// Erstellt einen neuen JavaScript-Context
    pub fn new() -> Self {
        Self {
            executed_scripts: Vec::new(),
            injected_scripts: HashMap::new(),
            event_listeners: HashMap::new(),
        }
    }

    /// Fügt eine JavaScript-Ausführung hinzu
    pub fn add_execution(&mut self, execution: JavaScriptExecution) {
        self.executed_scripts.push(execution);
        
        // Historie begrenzen (letzte 50 Ausführungen)
        if self.executed_scripts.len() > 50 {
            self.executed_scripts.remove(0);
        }
    }

    /// Injiziert ein Script
    pub fn inject_script(&mut self, name: String, script: String) {
        self.injected_scripts.insert(name, script);
    }

    /// Fügt einen Event-Listener hinzu
    pub fn add_event_listener(&mut self, event: String, handler: String) {
        self.event_listeners.entry(event).or_insert_with(Vec::new).push(handler);
    }

    /// Gibt injizierte Scripts zurück
    pub fn get_injected_scripts(&self) -> &HashMap<String, String> {
        &self.injected_scripts
    }

    /// Gibt Event-Listener zurück
    pub fn get_event_listeners(&self) -> &HashMap<String, Vec<String>> {
        &self.event_listeners
    }
}

impl Default for WebView2Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for JavaScriptContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = WebView2Engine::new();
        assert!(!engine.is_initialized);
        assert_eq!(engine.initialization_attempts, 0);
        assert!(engine.current_url.is_none());
        assert!(!engine.is_loading);
    }

    #[test]
    fn test_engine_with_config() {
        let config = WebView2Config::development();
        let engine = WebView2Engine::with_config(config.clone());
        assert_eq!(engine.config.debug.enable_dev_tools, config.debug.enable_dev_tools);
    }

    #[test]
    fn test_url_validation() {
        let engine = WebView2Engine::new();
        
        assert!(engine.is_valid_url("https://example.com"));
        assert!(engine.is_valid_url("http://example.com"));
        assert!(engine.is_valid_url("file:///C:/test.html"));
        assert!(engine.is_valid_url("data:text/html,<html></html>"));
        assert!(engine.is_valid_url("about:blank"));
        
        assert!(!engine.is_valid_url("invalid-url"));
        assert!(!engine.is_valid_url("ftp://example.com"));
        assert!(!engine.is_valid_url(""));
    }

    #[test]
    fn test_navigation_entry() {
        let entry = NavigationEntry {
            url: "https://example.com".to_string(),
            title: Some("Example".to_string()),
            timestamp: std::time::SystemTime::now(),
            navigation_type: NavigationType::Navigate,
            success: true,
            error: None,
        };
        
        assert_eq!(entry.url, "https://example.com");
        assert_eq!(entry.navigation_type, NavigationType::Navigate);
        assert!(entry.success);
    }

    #[test]
    fn test_javascript_context() {
        let mut context = JavaScriptContext::new();
        
        // Script injizieren
        context.inject_script("test".to_string(), "console.log('test')".to_string());
        assert_eq!(context.injected_scripts.len(), 1);
        
        // Event-Listener hinzufügen
        context.add_event_listener("click".to_string(), "handleClick()".to_string());
        assert_eq!(context.event_listeners.len(), 1);
        
        // JavaScript-Ausführung hinzufügen
        let execution = JavaScriptExecution {
            script: "alert('test')".to_string(),
            result: Some("OK".to_string()),
            error: None,
            timestamp: std::time::SystemTime::now(),
            execution_time_ms: 10,
        };
        
        context.add_execution(execution);
        assert_eq!(context.executed_scripts.len(), 1);
    }

    #[test]
    fn test_navigation_types() {
        assert_eq!(NavigationType::Navigate, NavigationType::Navigate);
        assert_ne!(NavigationType::Navigate, NavigationType::Reload);
    }

    #[tokio::test]
    async fn test_engine_configuration() {
        let mut engine = WebView2Engine::new();
        let config = WebView2Config::development();
        
        // Konfiguration sollte erfolgreich sein
        let result = engine.configure(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_diagnostic_report() {
        let mut engine = WebView2Engine::new();
        let report = engine.create_diagnostic_report();
        
        assert!(report.is_ok());
        let report = report.unwrap();
        
        assert!(report.contains_key("initialized"));
        assert!(report.contains_key("initialization_attempts"));
        assert!(report.contains_key("is_loading"));
        assert!(report.contains_key("config_summary"));
    }
}
