// 🔧 ZAKYX Browser Error Handlers
// Spezifische Error-Handler und Konvertierungen

use std::fmt;
use crate::error_system::types::{ZAKYXBrowserError, PluginErrorType, SecuritySeverity, StorageOperation};

/// Error-Handler für verschiedene externe Bibliotheken und Systeme
pub struct ErrorHandlers;

impl ErrorHandlers {
    /// Behandelt reqwest HTTP-Errors
    pub fn handle_reqwest_error(err: reqwest::Error) -> ZAKYXBrowserError {
        if err.is_timeout() {
            ZAKYXBrowserError::Timeout {
                seconds: 30,
                operation: "HTTP Request".to_string(),
                retry_possible: true,
            }
        } else if err.is_connect() {
            ZAKYXBrowserError::Network {
                message: "Verbindung fehlgeschlagen".to_string(),
                url: err.url().map(|u| u.to_string()),
                retry_possible: true,
            }
        } else if let Some(status) = err.status() {
            ZAKYXBrowserError::Http {
                status: status.as_u16(),
                message: format!("HTTP-Fehler: {}", status),
                url: err.url().map(|u| u.to_string()).unwrap_or_default(),
                retry_possible: status.is_server_error(),
            }
        } else {
            ZAKYXBrowserError::Network {
                message: err.to_string(),
                url: err.url().map(|u| u.to_string()),
                retry_possible: true,
            }
        }
    }

    /// Behandelt std::io::Error
    pub fn handle_io_error(err: std::io::Error) -> ZAKYXBrowserError {
        match err.kind() {
            std::io::ErrorKind::NotFound => ZAKYXBrowserError::FileNotFound {
                path: "unknown".to_string(),
                operation: "file operation".to_string(),
            },
            std::io::ErrorKind::PermissionDenied => ZAKYXBrowserError::Permission {
                message: err.to_string(),
                required_permission: "file access".to_string(),
            },
            std::io::ErrorKind::TimedOut => ZAKYXBrowserError::Timeout {
                seconds: 30,
                operation: "I/O operation".to_string(),
                retry_possible: true,
            },
            std::io::ErrorKind::ConnectionRefused => ZAKYXBrowserError::Network {
                message: "Verbindung verweigert".to_string(),
                url: None,
                retry_possible: true,
            },
            std::io::ErrorKind::ConnectionAborted => ZAKYXBrowserError::Network {
                message: "Verbindung abgebrochen".to_string(),
                url: None,
                retry_possible: true,
            },
            std::io::ErrorKind::BrokenPipe => ZAKYXBrowserError::Network {
                message: "Verbindung unterbrochen".to_string(),
                url: None,
                retry_possible: false,
            },
            std::io::ErrorKind::InvalidData => ZAKYXBrowserError::Serialization {
                message: "Ungültige Daten".to_string(),
                data_type: "unknown".to_string(),
            },
            _ => ZAKYXBrowserError::Internal {
                message: err.to_string(),
                error_code: Some(format!("IO_{:?}", err.kind())),
            },
        }
    }

    /// Behandelt Serde JSON-Errors
    pub fn handle_serde_error(err: serde_json::Error) -> ZAKYXBrowserError {
        ZAKYXBrowserError::Serialization {
            message: err.to_string(),
            data_type: "JSON".to_string(),
        }
    }

    /// Behandelt URL-Parsing-Errors
    pub fn handle_url_error(err: url::ParseError, url: &str) -> ZAKYXBrowserError {
        let reason = match err {
            url::ParseError::EmptyHost => "Leerer Host",
            url::ParseError::IdnaError => "Ungültiger Domain-Name",
            url::ParseError::InvalidDomainCharacter => "Ungültiges Zeichen im Domain-Namen",
            url::ParseError::InvalidIpv4Address => "Ungültige IPv4-Adresse",
            url::ParseError::InvalidIpv6Address => "Ungültige IPv6-Adresse",
            url::ParseError::InvalidPort => "Ungültiger Port",
            url::ParseError::Overflow => "URL zu lang",
            url::ParseError::RelativeUrlWithoutBase => "Relative URL ohne Basis",
            url::ParseError::RelativeUrlWithCannotBeABaseBase => "Ungültige Basis-URL",
            url::ParseError::SetHostOnCannotBeABaseUrl => "Host kann nicht gesetzt werden",
            // url::ParseError::TabOrNewlineInInput => "Ungültige Zeichen in URL", // Removed in newer url crate versions
            _ => "Unbekannter URL-Parsing-Fehler",
        };

        ZAKYXBrowserError::InvalidUrl {
            url: url.to_string(),
            reason: reason.to_string(),
        }
    }

    /// Behandelt Tauri-Errors
    pub fn handle_tauri_error(err: tauri::Error) -> ZAKYXBrowserError {
        match err {
            tauri::Error::Runtime(runtime_err) => {
                ZAKYXBrowserError::Internal {
                    message: format!("Tauri Runtime-Fehler: {}", runtime_err),
                    error_code: Some("TAURI_RUNTIME".to_string()),
                }
            },
            // tauri::Error::CreateWebview(webview_err) => { // Variant may not exist in current Tauri version
            //     ZAKYXBrowserError::WebView {
            //         message: format!("WebView-Erstellungsfehler: {}", webview_err),
            //         webview_id: None,
            //         recoverable: true,
            //     }
            // },
            tauri::Error::WindowLabelAlreadyExists(label) => {
                ZAKYXBrowserError::Ui {
                    message: format!("Fenster-Label bereits vorhanden: {}", label),
                    component: "window".to_string(),
                    recoverable: true,
                }
            },
            tauri::Error::WebviewLabelAlreadyExists(label) => {
                ZAKYXBrowserError::WebView {
                    message: format!("WebView-Label bereits vorhanden: {}", label),
                    webview_id: Some(label),
                    recoverable: true,
                }
            },
            tauri::Error::AssetNotFound(path) => {
                ZAKYXBrowserError::FileNotFound {
                    path: path.to_string(),
                    operation: "asset loading".to_string(),
                }
            },
            _ => {
                ZAKYXBrowserError::Internal {
                    message: err.to_string(),
                    error_code: Some("TAURI_UNKNOWN".to_string()),
                }
            }
        }
    }

    /// Behandelt Windows API-Errors (falls verfügbar)
    #[cfg(target_os = "windows")]
    pub fn handle_windows_error(error_code: u32) -> ZAKYXBrowserError {
        let message = match error_code {
            2 => "Datei nicht gefunden",
            3 => "Pfad nicht gefunden", 
            5 => "Zugriff verweigert",
            6 => "Ungültiges Handle",
            8 => "Nicht genügend Speicher",
            87 => "Ungültiger Parameter",
            123 => "Ungültiger Dateiname",
            183 => "Datei bereits vorhanden",
            _ => "Unbekannter Windows-Fehler",
        };

        match error_code {
            2 | 3 => ZAKYXBrowserError::FileNotFound {
                path: "unknown".to_string(),
                operation: "windows operation".to_string(),
            },
            5 => ZAKYXBrowserError::Permission {
                message: message.to_string(),
                required_permission: "windows access".to_string(),
            },
            _ => ZAKYXBrowserError::Internal {
                message: format!("{} (Code: {})", message, error_code),
                error_code: Some(format!("WIN32_{}", error_code)),
            },
        }
    }

    /// Behandelt Plugin-spezifische Errors
    pub fn handle_plugin_error(plugin_id: &str, error_message: &str, context: &str) -> ZAKYXBrowserError {
        let error_type = if error_message.contains("load") || error_message.contains("Load") {
            PluginErrorType::LoadError
        } else if error_message.contains("permission") || error_message.contains("Permission") {
            PluginErrorType::PermissionError
        } else if error_message.contains("config") || error_message.contains("Config") {
            PluginErrorType::ConfigError
        } else if error_message.contains("version") || error_message.contains("Version") {
            PluginErrorType::VersionError
        } else if error_message.contains("dependency") || error_message.contains("Dependency") {
            PluginErrorType::DependencyError
        } else if error_message.contains("api") || error_message.contains("API") {
            PluginErrorType::ApiError
        } else if error_message.contains("execution") || error_message.contains("runtime") {
            PluginErrorType::ExecutionError
        } else {
            PluginErrorType::Unknown
        };

        ZAKYXBrowserError::Plugin {
            plugin_id: plugin_id.to_string(),
            message: format!("{}: {}", context, error_message),
            error_type,
        }
    }

    /// Behandelt Security-bezogene Errors
    pub fn handle_security_error(message: &str, blocked_action: &str) -> ZAKYXBrowserError {
        let severity = if message.contains("critical") || message.contains("Critical") {
            SecuritySeverity::Critical
        } else if message.contains("high") || message.contains("High") {
            SecuritySeverity::High
        } else if message.contains("medium") || message.contains("Medium") {
            SecuritySeverity::Medium
        } else {
            SecuritySeverity::Low
        };

        ZAKYXBrowserError::Security {
            message: message.to_string(),
            severity,
            blocked_action: blocked_action.to_string(),
        }
    }

    /// Behandelt Storage/Database-Errors
    pub fn handle_storage_error(message: &str, operation: StorageOperation) -> ZAKYXBrowserError {
        let recoverable = match operation {
            StorageOperation::Read | StorageOperation::List => true,
            StorageOperation::Write | StorageOperation::Delete => false,
            StorageOperation::Initialize => false,
            _ => true,
        };

        ZAKYXBrowserError::Storage {
            message: message.to_string(),
            operation,
            recoverable,
        }
    }

    /// Behandelt WebView-spezifische Errors
    pub fn handle_webview_error(message: &str, webview_id: Option<&str>) -> ZAKYXBrowserError {
        let recoverable = !message.contains("fatal") && 
                         !message.contains("crashed") && 
                         !message.contains("terminated");

        ZAKYXBrowserError::WebView {
            message: message.to_string(),
            webview_id: webview_id.map(|s| s.to_string()),
            recoverable,
        }
    }

    /// Behandelt Proxy-spezifische Errors
    pub fn handle_proxy_error(message: &str, proxy_url: Option<&str>, target_url: &str) -> ZAKYXBrowserError {
        ZAKYXBrowserError::Proxy {
            message: message.to_string(),
            proxy_url: proxy_url.map(|s| s.to_string()),
            target_url: target_url.to_string(),
        }
    }

    /// Behandelt Navigation-Errors
    pub fn handle_navigation_error(message: &str, url: &str, error_code: Option<i32>) -> ZAKYXBrowserError {
        ZAKYXBrowserError::Navigation {
            message: message.to_string(),
            url: url.to_string(),
            error_code,
        }
    }

    /// Behandelt Certificate-Errors
    pub fn handle_certificate_error(domain: &str, certificate_error: &str) -> ZAKYXBrowserError {
        let message = if certificate_error.contains("expired") {
            "Zertifikat abgelaufen"
        } else if certificate_error.contains("invalid") {
            "Ungültiges Zertifikat"
        } else if certificate_error.contains("untrusted") {
            "Nicht vertrauenswürdiges Zertifikat"
        } else if certificate_error.contains("mismatch") {
            "Zertifikat stimmt nicht mit Domain überein"
        } else {
            "Zertifikatsfehler"
        };

        ZAKYXBrowserError::Certificate {
            domain: domain.to_string(),
            message: message.to_string(),
            certificate_error: certificate_error.to_string(),
        }
    }

    /// Behandelt Content-Size-Limit-Errors
    pub fn handle_content_size_error(size: usize, limit: usize, url: &str) -> ZAKYXBrowserError {
        ZAKYXBrowserError::ContentSizeLimit {
            size,
            limit,
            url: url.to_string(),
        }
    }

    /// Behandelt Initialization-Errors
    pub fn handle_initialization_error(message: &str, component: &str, critical: bool) -> ZAKYXBrowserError {
        ZAKYXBrowserError::Initialization {
            message: message.to_string(),
            component: component.to_string(),
            critical,
        }
    }

    /// Behandelt Feature-Not-Available-Errors
    pub fn handle_feature_unavailable_error(feature: &str, reason: &str) -> ZAKYXBrowserError {
        ZAKYXBrowserError::FeatureNotAvailable {
            feature: feature.to_string(),
            reason: reason.to_string(),
        }
    }

    /// Behandelt Resource-Unavailable-Errors
    pub fn handle_resource_unavailable_error(resource: &str, reason: &str) -> ZAKYXBrowserError {
        ZAKYXBrowserError::ResourceUnavailable {
            resource: resource.to_string(),
            reason: reason.to_string(),
        }
    }

    /// Allgemeine Error-Klassifizierung basierend auf Error-Message
    pub fn classify_error_by_message(message: &str) -> ZAKYXBrowserError {
        let lower_message = message.to_lowercase();

        if lower_message.contains("network") || lower_message.contains("connection") {
            ZAKYXBrowserError::network_error(message)
        } else if lower_message.contains("timeout") || lower_message.contains("timed out") {
            ZAKYXBrowserError::timeout_error(30, "unknown operation")
        } else if lower_message.contains("permission") || lower_message.contains("access denied") {
            ZAKYXBrowserError::Permission {
                message: message.to_string(),
                required_permission: "unknown".to_string(),
            }
        } else if lower_message.contains("file not found") || lower_message.contains("no such file") {
            ZAKYXBrowserError::FileNotFound {
                path: "unknown".to_string(),
                operation: "unknown".to_string(),
            }
        } else if lower_message.contains("plugin") {
            ZAKYXBrowserError::plugin_error("unknown", message, PluginErrorType::Unknown)
        } else if lower_message.contains("security") || lower_message.contains("blocked") {
            ZAKYXBrowserError::security_error(message, SecuritySeverity::Medium, "unknown action")
        } else if lower_message.contains("config") || lower_message.contains("configuration") {
            ZAKYXBrowserError::config_error(message)
        } else {
            ZAKYXBrowserError::Internal {
                message: message.to_string(),
                error_code: None,
            }
        }
    }
}

/// Automatische From-Implementierungen für häufige Error-Types
impl From<reqwest::Error> for ZAKYXBrowserError {
    fn from(err: reqwest::Error) -> Self {
        ErrorHandlers::handle_reqwest_error(err)
    }
}

impl From<std::io::Error> for ZAKYXBrowserError {
    fn from(err: std::io::Error) -> Self {
        ErrorHandlers::handle_io_error(err)
    }
}

impl From<serde_json::Error> for ZAKYXBrowserError {
    fn from(err: serde_json::Error) -> Self {
        ErrorHandlers::handle_serde_error(err)
    }
}

impl From<tauri::Error> for ZAKYXBrowserError {
    fn from(err: tauri::Error) -> Self {
        ErrorHandlers::handle_tauri_error(err)
    }
}

impl From<tokio::time::error::Elapsed> for ZAKYXBrowserError {
    fn from(_err: tokio::time::error::Elapsed) -> Self {
        ZAKYXBrowserError::Timeout {
            seconds: 30,
            operation: "async operation".to_string(),
            retry_possible: true,
        }
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ZAKYXBrowserError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        ZAKYXBrowserError::Internal {
            message: err.to_string(),
            error_code: None,
        }
    }
}

/// Error-Context für erweiterte Error-Informationen
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Kontext-Beschreibung
    pub description: String,
    
    /// Komponente, die den Fehler verursacht hat
    pub component: String,
    
    /// Operation, die fehlgeschlagen ist
    pub operation: String,
    
    /// Zusätzliche Metadaten
    pub metadata: std::collections::HashMap<String, String>,
    
    /// Stack-Trace (falls verfügbar)
    pub stack_trace: Option<String>,
    
    /// Zeitstempel
    pub timestamp: std::time::SystemTime,
}

impl ErrorContext {
    /// Erstellt einen neuen Error-Context
    pub fn new(description: impl Into<String>, component: impl Into<String>, operation: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            component: component.into(),
            operation: operation.into(),
            metadata: std::collections::HashMap::new(),
            stack_trace: None,
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Fügt Metadaten hinzu
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Fügt Stack-Trace hinzu
    pub fn with_stack_trace(mut self, stack_trace: impl Into<String>) -> Self {
        self.stack_trace = Some(stack_trace.into());
        self
    }

    /// Konvertiert zu einem ZAKYXBrowserError mit Context
    pub fn into_error(self, base_error: ZAKYXBrowserError) -> ZAKYXBrowserError {
        // Erweitere die Error-Message mit Context-Informationen
        match base_error {
            ZAKYXBrowserError::Internal { message, error_code } => {
                ZAKYXBrowserError::Internal {
                    message: format!("{} [Context: {} in {} during {}]", 
                        message, self.description, self.component, self.operation),
                    error_code,
                }
            },
            _ => base_error, // Für andere Error-Types bleibt der Fehler unverändert
        }
    }
}

/// Result-Type mit Error-Context
pub type ContextualResult<T> = Result<T, (ZAKYXBrowserError, ErrorContext)>;

/// Hilfsmakro für Error-Handling mit Context
#[macro_export]
macro_rules! error_with_context {
    ($error:expr, $description:expr, $component:expr, $operation:expr) => {
        {
            let context = ErrorContext::new($description, $component, $operation);
            let contextual_error = context.clone().into_error($error);
            Err((contextual_error, context))
        }
    };
    
    ($error:expr, $description:expr, $component:expr, $operation:expr, $($key:expr => $value:expr),*) => {
        {
            let mut context = ErrorContext::new($description, $component, $operation);
            $(
                context = context.with_metadata($key, $value);
            )*
            let contextual_error = context.clone().into_error($error);
            Err((contextual_error, context))
        }
    };
}

/// Error-Handler-Registry für Custom-Handler
#[derive(Default)]
pub struct ErrorHandlerRegistry {
    /// Custom Error-Handler
    handlers: std::collections::HashMap<String, Box<dyn Fn(&str) -> ZAKYXBrowserError + Send + Sync>>,
}

impl std::fmt::Debug for ErrorHandlerRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErrorHandlerRegistry")
            .field("handler_count", &self.handlers.len())
            .field("handler_names", &self.handlers.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl ErrorHandlerRegistry {
    /// Erstellt eine neue Handler-Registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Registriert einen Custom-Handler
    pub fn register_handler<F>(&mut self, name: String, handler: F) 
    where
        F: Fn(&str) -> ZAKYXBrowserError + Send + Sync + 'static,
    {
        self.handlers.insert(name, Box::new(handler));
    }

    /// Verwendet einen registrierten Handler
    pub fn handle_with(&self, handler_name: &str, message: &str) -> Option<ZAKYXBrowserError> {
        self.handlers.get(handler_name).map(|handler| handler(message))
    }

    /// Gibt alle registrierten Handler-Namen zurück
    pub fn get_handler_names(&self) -> Vec<&String> {
        self.handlers.keys().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_handling() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let zakyx_error = ErrorHandlers::handle_io_error(io_error);
        
        if let ZAKYXBrowserError::FileNotFound { path, operation } = zakyx_error {
            assert_eq!(path, "unknown");
            assert_eq!(operation, "file operation");
        } else {
            panic!("Expected FileNotFound error");
        }
    }

    #[test]
    fn test_plugin_error_handling() {
        let error = ErrorHandlers::handle_plugin_error("test-plugin", "Failed to load plugin", "initialization");
        
        if let ZAKYXBrowserError::Plugin { plugin_id, message, error_type } = error {
            assert_eq!(plugin_id, "test-plugin");
            assert!(message.contains("initialization"));
            assert_eq!(error_type, PluginErrorType::LoadError);
        } else {
            panic!("Expected Plugin error");
        }
    }

    #[test]
    fn test_security_error_handling() {
        let error = ErrorHandlers::handle_security_error("Critical security violation", "file access");
        
        if let ZAKYXBrowserError::Security { message, severity, blocked_action } = error {
            assert!(message.contains("Critical"));
            assert_eq!(severity, SecuritySeverity::Critical);
            assert_eq!(blocked_action, "file access");
        } else {
            panic!("Expected Security error");
        }
    }

    #[test]
    fn test_error_classification() {
        let network_error = ErrorHandlers::classify_error_by_message("Network connection failed");
        assert!(matches!(network_error, ZAKYXBrowserError::Network { .. }));
        
        let timeout_error = ErrorHandlers::classify_error_by_message("Operation timed out");
        assert!(matches!(timeout_error, ZAKYXBrowserError::Timeout { .. }));
        
        let permission_error = ErrorHandlers::classify_error_by_message("Permission denied");
        assert!(matches!(permission_error, ZAKYXBrowserError::Permission { .. }));
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new("Test error", "test_component", "test_operation")
            .with_metadata("key1", "value1")
            .with_metadata("key2", "value2");
        
        assert_eq!(context.description, "Test error");
        assert_eq!(context.component, "test_component");
        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.metadata.len(), 2);
        assert_eq!(context.metadata.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_error_handler_registry() {
        let mut registry = ErrorHandlerRegistry::new();
        
        registry.register_handler("custom_handler".to_string(), |message| {
            ZAKYXBrowserError::Internal {
                message: format!("Custom: {}", message),
                error_code: Some("CUSTOM".to_string()),
            }
        });
        
        let error = registry.handle_with("custom_handler", "test message");
        assert!(error.is_some());
        
        if let Some(ZAKYXBrowserError::Internal { message, error_code }) = error {
            assert!(message.contains("Custom: test message"));
            assert_eq!(error_code, Some("CUSTOM".to_string()));
        } else {
            panic!("Expected Internal error");
        }
    }

    #[test]
    fn test_from_implementations() {
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
        let zakyx_error: ZAKYXBrowserError = io_error.into();
        
        assert!(matches!(zakyx_error, ZAKYXBrowserError::Permission { .. }));
    }

    #[test]
    fn test_webview_error_handling() {
        let recoverable_error = ErrorHandlers::handle_webview_error("Navigation failed", Some("main-webview"));
        if let ZAKYXBrowserError::WebView { recoverable, webview_id, .. } = recoverable_error {
            assert!(recoverable);
            assert_eq!(webview_id, Some("main-webview".to_string()));
        } else {
            panic!("Expected WebView error");
        }
        
        let fatal_error = ErrorHandlers::handle_webview_error("WebView crashed", None);
        if let ZAKYXBrowserError::WebView { recoverable, .. } = fatal_error {
            assert!(!recoverable);
        } else {
            panic!("Expected WebView error");
        }
    }

    #[test]
    fn test_content_size_error() {
        let error = ErrorHandlers::handle_content_size_error(1024000, 512000, "https://example.com");
        
        if let ZAKYXBrowserError::ContentSizeLimit { size, limit, url } = error {
            assert_eq!(size, 1024000);
            assert_eq!(limit, 512000);
            assert_eq!(url, "https://example.com");
        } else {
            panic!("Expected ContentSizeLimit error");
        }
    }
}