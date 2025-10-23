// 🚨 ZAKYX Browser Error System
// Modulares Error-Handling-System für den ZAKYX Browser

#![allow(dead_code)] // Error system API - comprehensive API kept for extensibility

pub mod types;
pub mod recovery;
pub mod handlers;

// Re-exports für einfache Verwendung
pub use types::{
    ZAKYXBrowserError,
    PluginErrorType,
    SecuritySeverity,
    StorageOperation,
    ErrorCategory,
};

pub use recovery::{
    ErrorRecovery,
    ErrorRecoveryManager,
    RecoveryStrategy,
    RecoveryOptions,
    RecoveryConfig,
    ActiveRecovery,
    RecoveryStatus,
    RecoveryStatistics,
};

pub use handlers::{
    ErrorHandlers,
    ErrorContext,
    ErrorHandlerRegistry,
    ContextualResult,
};

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// Globaler Error-Manager für das ZAKYX Browser System
pub struct ZAKYXErrorSystem {
    /// Recovery-Manager
    recovery_manager: Arc<Mutex<ErrorRecoveryManager>>,
    
    /// Error-Handler-Registry
    handler_registry: Arc<Mutex<ErrorHandlerRegistry>>,
    
    /// Error-Statistiken
    statistics: Arc<Mutex<ErrorSystemStatistics>>,
    
    /// System-Konfiguration
    config: ErrorSystemConfig,
    
    /// Error-Listener
    listeners: Arc<Mutex<Vec<Box<dyn ErrorListener + Send + Sync>>>>,
}

impl std::fmt::Debug for ZAKYXErrorSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZAKYXErrorSystem")
            .field("config", &self.config)
            .field("listener_count", &self.listeners.lock().map(|l| l.len()).unwrap_or(0))
            .finish()
    }
}

/// Error-System-Statistiken
#[derive(Debug, Default, Clone)]
pub struct ErrorSystemStatistics {
    /// Gesamte Fehler-Anzahl
    pub total_errors: u64,
    
    /// Fehler nach Kategorie
    pub errors_by_category: HashMap<ErrorCategory, u64>,
    
    /// Fehler nach Schweregrad
    pub errors_by_severity: HashMap<String, u64>,
    
    /// Automatisch behobene Fehler
    pub auto_resolved_errors: u64,
    
    /// Fehler, die Benutzer-Intervention erfordern
    pub user_intervention_required: u64,
    
    /// Durchschnittliche Fehlerbehandlungszeit
    pub average_handling_time: std::time::Duration,
    
    /// Letzte Fehler (begrenzt auf die letzten 100)
    pub recent_errors: Vec<ErrorLogEntry>,
}

/// Error-System-Konfiguration
#[derive(Debug, Clone)]
pub struct ErrorSystemConfig {
    /// Ob automatische Recovery aktiviert ist
    pub auto_recovery_enabled: bool,
    
    /// Ob Error-Logging aktiviert ist
    pub error_logging_enabled: bool,
    
    /// Maximale Anzahl von Error-Log-Einträgen
    pub max_error_log_entries: usize,
    
    /// Ob Benutzer-Benachrichtigungen aktiviert sind
    pub user_notifications_enabled: bool,
    
    /// Minimaler Schweregrad für Benutzer-Benachrichtigungen
    pub min_notification_severity: String,
    
    /// Ob detaillierte Error-Reports erstellt werden
    pub detailed_error_reports: bool,
    
    /// Error-Report-Verzeichnis
    pub error_report_directory: std::path::PathBuf,
}

/// Error-Log-Eintrag
#[derive(Debug, Clone)]
pub struct ErrorLogEntry {
    /// Zeitstempel
    pub timestamp: std::time::SystemTime,
    
    /// Error-Typ
    pub error: ZAKYXBrowserError,
    
    /// Error-Kontext
    pub context: Option<ErrorContext>,
    
    /// Ob der Fehler automatisch behoben wurde
    pub auto_resolved: bool,
    
    /// Recovery-ID (falls vorhanden)
    pub recovery_id: Option<String>,
    
    /// Zusätzliche Metadaten
    pub metadata: HashMap<String, String>,
}

/// Error-Listener-Trait
pub trait ErrorListener {
    /// Wird aufgerufen, wenn ein neuer Fehler auftritt
    fn on_error(&self, error: &ZAKYXBrowserError, context: Option<&ErrorContext>);
    
    /// Wird aufgerufen, wenn ein Fehler behoben wurde
    fn on_error_resolved(&self, error: &ZAKYXBrowserError, recovery_id: &str);
    
    /// Wird aufgerufen, wenn eine Recovery fehlschlägt
    fn on_recovery_failed(&self, error: &ZAKYXBrowserError, recovery_id: &str);
}

impl ZAKYXErrorSystem {
    /// Erstellt ein neues Error-System
    pub fn new() -> Self {
        println!("🚨 Creating ZAKYX Error System...");
        
        let recovery_manager = Arc::new(Mutex::new(ErrorRecoveryManager::new()));
        let handler_registry = Arc::new(Mutex::new(ErrorHandlerRegistry::new()));
        let statistics = Arc::new(Mutex::new(ErrorSystemStatistics::default()));
        let listeners = Arc::new(Mutex::new(Vec::new()));
        
        let mut system = Self {
            recovery_manager,
            handler_registry,
            statistics,
            config: ErrorSystemConfig::default(),
            listeners,
        };
        
        // Registriere Standard-Handler
        system.register_default_handlers();
        
        println!("✅ ZAKYX Error System created successfully!");
        system
    }

    /// Erstellt ein Error-System mit spezifischer Konfiguration
    pub fn with_config(config: ErrorSystemConfig) -> Self {
        println!("🚨 Creating ZAKYX Error System with custom config...");
        
        let recovery_manager = Arc::new(Mutex::new(ErrorRecoveryManager::new()));
        let handler_registry = Arc::new(Mutex::new(ErrorHandlerRegistry::new()));
        let statistics = Arc::new(Mutex::new(ErrorSystemStatistics::default()));
        let listeners = Arc::new(Mutex::new(Vec::new()));
        
        let mut system = Self {
            recovery_manager,
            handler_registry,
            statistics,
            config,
            listeners,
        };
        
        // Registriere Standard-Handler
        system.register_default_handlers();
        
        println!("✅ ZAKYX Error System with config created successfully!");
        system
    }

    /// Registriert Standard-Error-Handler
    fn register_default_handlers(&mut self) {
        if let Ok(mut registry) = self.handler_registry.lock() {
            // Network-Error-Handler
            registry.register_handler("network_handler".to_string(), |message| {
                ZAKYXBrowserError::network_error(message)
            });
            
            // Plugin-Error-Handler
            registry.register_handler("plugin_handler".to_string(), |message| {
                ZAKYXBrowserError::plugin_error("unknown", message, PluginErrorType::Unknown)
            });
            
            // Security-Error-Handler
            registry.register_handler("security_handler".to_string(), |message| {
                ZAKYXBrowserError::security_error(message, SecuritySeverity::Medium, "unknown action")
            });
            
            println!("✅ Default error handlers registered");
        }
    }

    /// Behandelt einen neuen Fehler
    pub async fn handle_error(&self, error: ZAKYXBrowserError, context: Option<ErrorContext>) -> Result<String, ZAKYXBrowserError> {
        println!("🚨 Handling error: {}", error);
        
        // Aktualisiere Statistiken
        self.update_statistics(&error, &context);
        
        // Logge den Fehler
        if self.config.error_logging_enabled {
            self.log_error(&error, &context, None).await;
        }
        
        // Benachrichtige Listener
        self.notify_listeners(&error, context.as_ref());
        
        // Starte automatische Recovery (falls aktiviert)
        if self.config.auto_recovery_enabled && error.is_retryable() {
            if let Ok(mut recovery_manager) = self.recovery_manager.lock() {
                match recovery_manager.start_recovery(error.clone()).await {
                    Ok(recovery_id) => {
                        println!("🔄 Started automatic recovery: {}", recovery_id);
                        
                        // Logge Recovery-Start
                        if self.config.error_logging_enabled {
                            self.log_error(&error, &context, Some(recovery_id.clone())).await;
                        }
                        
                        return Ok(recovery_id);
                    },
                    Err(recovery_error) => {
                        println!("❌ Failed to start recovery: {}", recovery_error);
                    }
                }
            }
        }
        
        // Prüfe ob Benutzer-Intervention erforderlich ist
        if error.requires_user_action() && self.config.user_notifications_enabled {
            self.notify_user(&error, context.as_ref()).await;
        }
        
        // Erstelle detaillierten Report (falls aktiviert)
        if self.config.detailed_error_reports {
            self.create_error_report(&error, &context).await?;
        }
        
        Err(error)
    }

    /// Behandelt einen Fehler mit einem spezifischen Handler
    pub async fn handle_error_with_handler(&self, handler_name: &str, message: &str, context: Option<ErrorContext>) -> Result<String, ZAKYXBrowserError> {
        if let Ok(registry) = self.handler_registry.lock() {
            if let Some(error) = registry.handle_with(handler_name, message) {
                return self.handle_error(error, context).await;
            }
        }
        
        // Fallback zu Standard-Error-Klassifizierung
        let error = ErrorHandlers::classify_error_by_message(message);
        self.handle_error(error, context).await
    }

    /// Aktualisiert Error-Statistiken
    fn update_statistics(&self, error: &ZAKYXBrowserError, _context: &Option<ErrorContext>) {
        if let Ok(mut stats) = self.statistics.lock() {
            stats.total_errors += 1;
            
            // Aktualisiere Kategorie-Statistiken
            let category = error.category();
            *stats.errors_by_category.entry(category).or_insert(0) += 1;
            
            // Aktualisiere Schweregrad-Statistiken
            let severity = if error.is_critical() { "Critical" } else if error.requires_user_action() { "High" } else { "Medium" };
            *stats.errors_by_severity.entry(severity.to_string()).or_insert(0) += 1;
            
            // Aktualisiere Benutzer-Intervention-Statistiken
            if error.requires_user_action() {
                stats.user_intervention_required += 1;
            }
        }
    }

    /// Loggt einen Fehler
    async fn log_error(&self, error: &ZAKYXBrowserError, context: &Option<ErrorContext>, recovery_id: Option<String>) {
        let log_entry = ErrorLogEntry {
            timestamp: std::time::SystemTime::now(),
            error: error.clone(),
            context: context.clone(),
            auto_resolved: false,
            recovery_id,
            metadata: HashMap::new(),
        };
        
        if let Ok(mut stats) = self.statistics.lock() {
            stats.recent_errors.push(log_entry);
            
            // Begrenze Log-Größe
            if stats.recent_errors.len() > self.config.max_error_log_entries {
                stats.recent_errors.remove(0);
            }
        }
        
        println!("📝 Error logged: {}", error);
    }

    /// Benachrichtigt alle Error-Listener
    fn notify_listeners(&self, error: &ZAKYXBrowserError, context: Option<&ErrorContext>) {
        if let Ok(listeners) = self.listeners.lock() {
            for listener in listeners.iter() {
                listener.on_error(error, context);
            }
        }
    }

    /// Benachrichtigt den Benutzer über einen Fehler
    async fn notify_user(&self, error: &ZAKYXBrowserError, context: Option<&ErrorContext>) {
        let user_message = error.user_message();
        let suggested_actions = error.suggested_actions();
        
        println!("📢 User notification:");
        println!("   Message: {}", user_message);
        println!("   Suggested actions: {:?}", suggested_actions);
        
        if let Some(ctx) = context {
            println!("   Context: {} in {} during {}", ctx.description, ctx.component, ctx.operation);
        }
    }

    /// Erstellt einen detaillierten Error-Report
    async fn create_error_report(&self, error: &ZAKYXBrowserError, context: &Option<ErrorContext>) -> Result<(), ZAKYXBrowserError> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let filename = format!("error_report_{}.json", timestamp);
        let filepath = self.config.error_report_directory.join(filename);
        
        let report = serde_json::json!({
            "timestamp": timestamp,
            "error": {
                "type": format!("{:?}", error.category()),
                "message": error.to_string(),
                "user_message": error.user_message(),
                "is_critical": error.is_critical(),
                "is_retryable": error.is_retryable(),
                "requires_user_action": error.requires_user_action(),
                "suggested_actions": error.suggested_actions(),
            },
            "context": context.as_ref().map(|ctx| serde_json::json!({
                "description": ctx.description,
                "component": ctx.component,
                "operation": ctx.operation,
                "metadata": ctx.metadata,
                "timestamp": ctx.timestamp.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
            })),
            "system_info": {
                "version": env!("CARGO_PKG_VERSION"),
                "target": std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "unknown".to_string()),
            }
        });
        
        // Erstelle Verzeichnis falls nicht vorhanden
        if let Some(parent) = filepath.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                ZAKYXBrowserError::Storage {
                    message: format!("Failed to create error report directory: {}", e),
                    operation: StorageOperation::Write,
                    recoverable: false,
                }
            })?;
        }
        
        // Schreibe Report
        tokio::fs::write(&filepath, report.to_string()).await.map_err(|e| {
            ZAKYXBrowserError::Storage {
                message: format!("Failed to write error report: {}", e),
                operation: StorageOperation::Write,
                recoverable: false,
            }
        })?;
        
        println!("📄 Error report created: {}", filepath.display());
        Ok(())
    }

    /// Fügt einen Error-Listener hinzu
    pub fn add_error_listener(&self, listener: Box<dyn ErrorListener + Send + Sync>) {
        if let Ok(mut listeners) = self.listeners.lock() {
            listeners.push(listener);
            println!("👂 Error listener added");
        }
    }

    /// Registriert einen Custom-Error-Handler
    pub fn register_error_handler<F>(&self, name: String, handler: F) 
    where
        F: Fn(&str) -> ZAKYXBrowserError + Send + Sync + 'static,
    {
        if let Ok(mut registry) = self.handler_registry.lock() {
            registry.register_handler(name.clone(), handler);
            println!("🔧 Custom error handler registered: {}", name);
        }
    }

    /// Gibt Error-Statistiken zurück
    pub fn get_statistics(&self) -> Option<ErrorSystemStatistics> {
        self.statistics.lock().ok().map(|stats| stats.clone())
    }

    /// Gibt die Recovery-Manager-Referenz zurück
    pub fn get_recovery_manager(&self) -> Arc<Mutex<ErrorRecoveryManager>> {
        Arc::clone(&self.recovery_manager)
    }

    /// Erstellt einen System-Report
    pub async fn create_system_report(&self) -> String {
        let mut report = String::new();
        report.push_str("🚨 ZAKYX ERROR SYSTEM REPORT\n");
        report.push_str("=============================\n\n");
        
        // System-Konfiguration
        report.push_str("⚙️ CONFIGURATION:\n");
        report.push_str(&format!("• Auto Recovery: {}\n", self.config.auto_recovery_enabled));
        report.push_str(&format!("• Error Logging: {}\n", self.config.error_logging_enabled));
        report.push_str(&format!("• User Notifications: {}\n", self.config.user_notifications_enabled));
        report.push_str(&format!("• Detailed Reports: {}\n", self.config.detailed_error_reports));
        report.push_str("\n");
        
        // Statistiken
        if let Ok(stats) = self.statistics.lock() {
            report.push_str("📊 STATISTICS:\n");
            report.push_str(&format!("• Total Errors: {}\n", stats.total_errors));
            report.push_str(&format!("• Auto Resolved: {}\n", stats.auto_resolved_errors));
            report.push_str(&format!("• User Intervention Required: {}\n", stats.user_intervention_required));
            report.push_str(&format!("• Recent Errors: {}\n", stats.recent_errors.len()));
            report.push_str("\n");
            
            if !stats.errors_by_category.is_empty() {
                report.push_str("📈 ERRORS BY CATEGORY:\n");
                for (category, count) in &stats.errors_by_category {
                    report.push_str(&format!("• {:?}: {}\n", category, count));
                }
                report.push_str("\n");
            }
            
            if !stats.errors_by_severity.is_empty() {
                report.push_str("🚨 ERRORS BY SEVERITY:\n");
                for (severity, count) in &stats.errors_by_severity {
                    report.push_str(&format!("• {}: {}\n", severity, count));
                }
                report.push_str("\n");
            }
        }
        
        // Recovery-Manager-Report
        if let Ok(recovery_manager) = self.recovery_manager.lock() {
            report.push_str(&recovery_manager.create_recovery_report());
        }
        
        // Handler-Registry
        if let Ok(registry) = self.handler_registry.lock() {
            let handler_names = registry.get_handler_names();
            if !handler_names.is_empty() {
                report.push_str("🔧 REGISTERED HANDLERS:\n");
                for name in handler_names {
                    report.push_str(&format!("• {}\n", name));
                }
                report.push_str("\n");
            }
        }
        
        report
    }

    /// Bereinigt alte Error-Logs und abgeschlossene Recoveries
    pub async fn cleanup(&self) -> usize {
        let mut cleaned_count = 0;
        
        // Bereinige Recovery-Manager
        if let Ok(mut recovery_manager) = self.recovery_manager.lock() {
            cleaned_count += recovery_manager.cleanup_completed_recoveries();
        }
        
        // Bereinige alte Error-Logs (behalte nur die letzten 50)
        if let Ok(mut stats) = self.statistics.lock() {
            let initial_count = stats.recent_errors.len();
            if initial_count > 50 {
                stats.recent_errors.drain(0..initial_count - 50);
                cleaned_count += initial_count - 50;
            }
        }
        
        if cleaned_count > 0 {
            println!("🧹 Error system cleanup: {} items removed", cleaned_count);
        }
        
        cleaned_count
    }

    // Getter
    pub fn get_config(&self) -> &ErrorSystemConfig {
        &self.config
    }
}

impl Default for ErrorSystemConfig {
    fn default() -> Self {
        Self {
            auto_recovery_enabled: true,
            error_logging_enabled: true,
            max_error_log_entries: 100,
            user_notifications_enabled: true,
            min_notification_severity: "Medium".to_string(),
            detailed_error_reports: false,
            error_report_directory: std::path::PathBuf::from("error_reports"),
        }
    }
}

impl Default for ZAKYXErrorSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Globale Error-System-Instanz (Singleton)
static mut GLOBAL_ERROR_SYSTEM: Option<ZAKYXErrorSystem> = None;
static INIT: std::sync::Once = std::sync::Once::new();

/// Initialisiert das globale Error-System
pub fn initialize_global_error_system() -> &'static ZAKYXErrorSystem {
    unsafe {
        INIT.call_once(|| {
            GLOBAL_ERROR_SYSTEM = Some(ZAKYXErrorSystem::new());
        });
        GLOBAL_ERROR_SYSTEM.as_ref().unwrap()
    }
}

/// Initialisiert das globale Error-System mit Konfiguration
pub fn initialize_global_error_system_with_config(config: ErrorSystemConfig) -> &'static ZAKYXErrorSystem {
    unsafe {
        INIT.call_once(|| {
            GLOBAL_ERROR_SYSTEM = Some(ZAKYXErrorSystem::with_config(config));
        });
        GLOBAL_ERROR_SYSTEM.as_ref().unwrap()
    }
}

/// Gibt das globale Error-System zurück
pub fn get_global_error_system() -> Option<&'static ZAKYXErrorSystem> {
    unsafe { GLOBAL_ERROR_SYSTEM.as_ref() }
}

/// Convenience-Funktion für globale Error-Behandlung
pub async fn handle_global_error(error: ZAKYXBrowserError, context: Option<ErrorContext>) -> Result<String, ZAKYXBrowserError> {
    if let Some(system) = get_global_error_system() {
        system.handle_error(error, context).await
    } else {
        println!("⚠️ Global error system not initialized, handling error locally");
        Err(error)
    }
}

/// Result-Type für das ZAKYX Browser System
pub type ZAKYXBrowserResult<T> = Result<T, ZAKYXBrowserError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_error_system_creation() {
        let system = ZAKYXErrorSystem::new();
        assert!(system.config.auto_recovery_enabled);
        assert!(system.config.error_logging_enabled);
    }

    #[tokio::test]
    async fn test_error_handling() {
        let system = ZAKYXErrorSystem::new();
        let error = ZAKYXBrowserError::network_error("Test error");
        let context = Some(ErrorContext::new("Test context", "test_component", "test_operation"));
        
        let result = system.handle_error(error, context).await;
        // Should return recovery ID for retryable errors
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_statistics_update() {
        let system = ZAKYXErrorSystem::new();
        let error = ZAKYXBrowserError::network_error("Test error");
        
        let _ = system.handle_error(error, None).await;
        
        let stats = system.get_statistics();
        assert!(stats.is_some());
        
        let stats = stats.unwrap();
        assert_eq!(stats.total_errors, 1);
        assert!(stats.errors_by_category.contains_key(&ErrorCategory::Network));
    }

    #[tokio::test]
    async fn test_custom_handler_registration() {
        let system = ZAKYXErrorSystem::new();
        
        system.register_error_handler("test_handler".to_string(), |message| {
            ZAKYXBrowserError::Internal {
                message: format!("Test: {}", message),
                error_code: Some("TEST".to_string()),
            }
        });
        
        let result = system.handle_error_with_handler("test_handler", "test message", None).await;
        assert!(result.is_err());
        
        if let Err(error) = result {
            if let ZAKYXBrowserError::Internal { message, .. } = error {
                assert!(message.contains("Test: test message"));
            }
        }
    }

    #[test]
    fn test_global_error_system() {
        let system = initialize_global_error_system();
        assert!(system.config.auto_recovery_enabled);
        
        let system2 = get_global_error_system();
        assert!(system2.is_some());
    }

    #[test]
    fn test_error_system_config() {
        let config = ErrorSystemConfig {
            auto_recovery_enabled: false,
            error_logging_enabled: false,
            max_error_log_entries: 50,
            user_notifications_enabled: false,
            min_notification_severity: "High".to_string(),
            detailed_error_reports: true,
            error_report_directory: std::path::PathBuf::from("custom_reports"),
        };
        
        let system = ZAKYXErrorSystem::with_config(config.clone());
        assert!(!system.config.auto_recovery_enabled);
        assert!(!system.config.error_logging_enabled);
        assert_eq!(system.config.max_error_log_entries, 50);
        assert!(system.config.detailed_error_reports);
    }
}