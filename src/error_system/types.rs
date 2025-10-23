// 🚨 ZAKYX Browser Error Types
// Zentrale Error-Definitionen für das gesamte System

use std::fmt;
use thiserror::Error;
use serde::{Serialize, Deserialize};

/// Zentraler Error-Type für das gesamte ZAKYX Browser System
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum ZAKYXBrowserError {
    // 🌐 NETWORK & HTTP ERRORS
    #[error("Netzwerkfehler: {message}")]
    Network { 
        message: String, 
        url: Option<String>,
        retry_possible: bool,
    },

    #[error("HTTP-Fehler {status}: {message}")]
    Http { 
        status: u16, 
        message: String, 
        url: String,
        retry_possible: bool,
    },

    #[error("Timeout nach {seconds} Sekunden")]
    Timeout { 
        seconds: u64,
        operation: String,
        retry_possible: bool,
    },

    #[error("Content-Größe überschreitet Limit: {size} bytes (Limit: {limit} bytes)")]
    ContentSizeLimit { 
        size: usize, 
        limit: usize,
        url: String,
    },

    // 🔧 CONFIGURATION ERRORS
    #[error("Konfigurationsfehler: {message}")]
    Config { 
        message: String,
        field: Option<String>,
        fix_suggestion: Option<String>,
    },

    #[error("Ungültige URL: {url}")]
    InvalidUrl { 
        url: String,
        reason: String,
    },

    // 📁 FILE & STORAGE ERRORS
    #[error("Datei nicht gefunden: {path}")]
    FileNotFound { 
        path: String,
        operation: String,
    },

    #[error("Keine Berechtigung für: {message}")]
    Permission { 
        message: String,
        required_permission: String,
    },

    #[error("Storage-Fehler bei {operation:?}: {message}")]
    Storage { 
        message: String,
        operation: StorageOperation,
        recoverable: bool,
    },

    #[error("Serialisierungsfehler: {message}")]
    Serialization { 
        message: String,
        data_type: String,
    },

    // 🖥️ UI & RENDERING ERRORS
    #[error("UI-Fehler: {message}")]
    Ui { 
        message: String,
        component: String,
        recoverable: bool,
    },

    #[error("WebView-Fehler: {message}")]
    WebView { 
        message: String,
        webview_id: Option<String>,
        recoverable: bool,
    },

    #[error("Rendering-Fehler: {message}")]
    Rendering { 
        message: String,
        element: Option<String>,
    },

    // 🔌 PLUGIN & EXTENSION ERRORS
    #[error("Plugin-Fehler '{plugin_id}': {message}")]
    Plugin { 
        plugin_id: String,
        message: String,
        error_type: PluginErrorType,
    },

    #[error("Plugin nicht gefunden: {plugin_id}")]
    PluginNotFound { 
        plugin_id: String,
    },

    #[error("Plugin-Validierung fehlgeschlagen: {message}")]
    PluginValidation { 
        plugin_id: String,
        message: String,
        validation_errors: Vec<String>,
    },

    // 🛡️ SECURITY ERRORS
    #[error("Sicherheitsfehler: {message}")]
    Security { 
        message: String,
        severity: SecuritySeverity,
        blocked_action: String,
    },

    #[error("Zertifikatsfehler für {domain}: {message}")]
    Certificate { 
        domain: String,
        message: String,
        certificate_error: String,
    },

    // 🌐 PROXY & NAVIGATION ERRORS
    #[error("Proxy-Fehler: {message}")]
    Proxy { 
        message: String,
        proxy_url: Option<String>,
        target_url: String,
    },

    #[error("Navigation-Fehler: {message}")]
    Navigation { 
        message: String,
        url: String,
        error_code: Option<i32>,
    },

    // 🔄 SYSTEM & GENERAL ERRORS
    #[error("Initialisierungsfehler: {message}")]
    Initialization { 
        message: String,
        component: String,
        critical: bool,
    },

    #[error("Interner Fehler: {message}")]
    Internal { 
        message: String,
        error_code: Option<String>,
    },

    #[error("Feature nicht verfügbar: {feature}")]
    FeatureNotAvailable { 
        feature: String,
        reason: String,
    },

    #[error("Ressource nicht verfügbar: {resource}")]
    ResourceUnavailable { 
        resource: String,
        reason: String,
    },
}

/// Plugin-Error-Typen
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginErrorType {
    /// Plugin konnte nicht geladen werden
    LoadError,
    
    /// Plugin-Ausführungsfehler
    ExecutionError,
    
    /// Plugin-Konfigurationsfehler
    ConfigError,
    
    /// Plugin-Abhängigkeitsfehler
    DependencyError,
    
    /// Plugin-Berechtigungsfehler
    PermissionError,
    
    /// Plugin-Versionsfehler
    VersionError,
    
    /// Plugin-API-Fehler
    ApiError,
    
    /// Unbekannter Plugin-Fehler
    Unknown,
}

/// Sicherheitsstufen für Security-Errors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecuritySeverity {
    /// Niedrige Sicherheitsstufe
    Low,
    
    /// Mittlere Sicherheitsstufe
    Medium,
    
    /// Hohe Sicherheitsstufe
    High,
    
    /// Kritische Sicherheitsstufe
    Critical,
}

/// Storage-Operationen
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StorageOperation {
    /// Lese-Operation
    Read,
    
    /// Schreib-Operation
    Write,
    
    /// Lösch-Operation
    Delete,
    
    /// List-Operation
    List,
    
    /// Initialisierungs-Operation
    Initialize,
    
    /// Backup-Operation
    Backup,
    
    /// Wiederherstellungs-Operation
    Restore,
    
    /// Synchronisations-Operation
    Sync,
}

/// Browser-spezifische Error-Kategorien
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Netzwerk-bezogene Fehler
    Network,
    
    /// Datei-System-Fehler
    FileSystem,
    
    /// UI/Rendering-Fehler
    UserInterface,
    
    /// Plugin-System-Fehler
    Plugin,
    
    /// Sicherheits-Fehler
    Security,
    
    /// Konfigurationsfehler
    Configuration,
    
    /// System-Fehler
    System,
    
    /// Unbekannte Fehler
    Unknown,
}

impl ZAKYXBrowserError {
    /// Gibt die Error-Kategorie zurück
    pub fn category(&self) -> ErrorCategory {
        match self {
            ZAKYXBrowserError::Network { .. } |
            ZAKYXBrowserError::Http { .. } |
            ZAKYXBrowserError::Timeout { .. } |
            ZAKYXBrowserError::ContentSizeLimit { .. } |
            ZAKYXBrowserError::Proxy { .. } |
            ZAKYXBrowserError::Navigation { .. } => ErrorCategory::Network,
            
            ZAKYXBrowserError::FileNotFound { .. } |
            ZAKYXBrowserError::Permission { .. } |
            ZAKYXBrowserError::Storage { .. } |
            ZAKYXBrowserError::Serialization { .. } => ErrorCategory::FileSystem,
            
            ZAKYXBrowserError::Ui { .. } |
            ZAKYXBrowserError::WebView { .. } |
            ZAKYXBrowserError::Rendering { .. } => ErrorCategory::UserInterface,
            
            ZAKYXBrowserError::Plugin { .. } |
            ZAKYXBrowserError::PluginNotFound { .. } |
            ZAKYXBrowserError::PluginValidation { .. } => ErrorCategory::Plugin,
            
            ZAKYXBrowserError::Security { .. } |
            ZAKYXBrowserError::Certificate { .. } => ErrorCategory::Security,
            
            ZAKYXBrowserError::Config { .. } |
            ZAKYXBrowserError::InvalidUrl { .. } => ErrorCategory::Configuration,
            
            ZAKYXBrowserError::Initialization { .. } |
            ZAKYXBrowserError::Internal { .. } |
            ZAKYXBrowserError::FeatureNotAvailable { .. } |
            ZAKYXBrowserError::ResourceUnavailable { .. } => ErrorCategory::System,
        }
    }

    /// Prüft ob der Fehler kritisch ist
    pub fn is_critical(&self) -> bool {
        match self {
            ZAKYXBrowserError::Security { severity: SecuritySeverity::Critical, .. } => true,
            ZAKYXBrowserError::Initialization { critical: true, .. } => true,
            ZAKYXBrowserError::Internal { .. } => true,
            _ => false,
        }
    }

    /// Prüft ob der Fehler wiederholbar ist
    pub fn is_retryable(&self) -> bool {
        match self {
            ZAKYXBrowserError::Network { retry_possible, .. } => *retry_possible,
            ZAKYXBrowserError::Http { retry_possible, .. } => *retry_possible,
            ZAKYXBrowserError::Timeout { retry_possible, .. } => *retry_possible,
            ZAKYXBrowserError::Ui { recoverable, .. } => *recoverable,
            ZAKYXBrowserError::WebView { recoverable, .. } => *recoverable,
            ZAKYXBrowserError::Storage { recoverable, .. } => *recoverable,
            ZAKYXBrowserError::Proxy { .. } => true, // Proxy kann verschiedene Strategien versuchen
            _ => false,
        }
    }

    /// Prüft ob Benutzer-Aktion erforderlich ist
    pub fn requires_user_action(&self) -> bool {
        match self {
            ZAKYXBrowserError::Permission { .. } => true,
            ZAKYXBrowserError::Security { .. } => true,
            ZAKYXBrowserError::Certificate { .. } => true,
            ZAKYXBrowserError::Config { .. } => true,
            ZAKYXBrowserError::PluginValidation { .. } => true,
            _ => false,
        }
    }

    /// Gibt eine benutzerfreundliche Fehlermeldung zurück
    pub fn user_message(&self) -> String {
        match self {
            ZAKYXBrowserError::Network { message, .. } => {
                format!("🌐 Netzwerkproblem: {}", message)
            },
            ZAKYXBrowserError::Http { status, message, .. } => {
                format!("🌐 Server-Fehler ({}): {}", status, message)
            },
            ZAKYXBrowserError::Timeout { operation, .. } => {
                format!("⏱️ Zeitüberschreitung bei: {}", operation)
            },
            ZAKYXBrowserError::FileNotFound { path, .. } => {
                format!("📁 Datei nicht gefunden: {}", path)
            },
            ZAKYXBrowserError::Permission { message, .. } => {
                format!("🔒 Berechtigung erforderlich: {}", message)
            },
            ZAKYXBrowserError::Plugin { plugin_id, message, .. } => {
                format!("🔌 Plugin-Problem ({}): {}", plugin_id, message)
            },
            ZAKYXBrowserError::Security { message, .. } => {
                format!("🛡️ Sicherheitswarnung: {}", message)
            },
            _ => self.to_string(),
        }
    }

    /// Gibt Lösungsvorschläge zurück
    pub fn suggested_actions(&self) -> Vec<String> {
        match self {
            ZAKYXBrowserError::Network { .. } => vec![
                "Internetverbindung prüfen".to_string(),
                "Proxy-Einstellungen überprüfen".to_string(),
                "Später erneut versuchen".to_string(),
            ],
            ZAKYXBrowserError::Http { status, .. } => match *status {
                404 => vec!["URL überprüfen".to_string(), "Link aktualisieren".to_string()],
                500..=599 => vec!["Später erneut versuchen".to_string(), "Server-Administrator kontaktieren".to_string()],
                _ => vec!["Seite neu laden".to_string()],
            },
            ZAKYXBrowserError::FileNotFound { .. } => vec![
                "Dateipfad überprüfen".to_string(),
                "Datei wiederherstellen".to_string(),
            ],
            ZAKYXBrowserError::Permission { .. } => vec![
                "Als Administrator ausführen".to_string(),
                "Dateiberechtigungen ändern".to_string(),
            ],
            ZAKYXBrowserError::Plugin { .. } => vec![
                "Plugin neu installieren".to_string(),
                "Plugin deaktivieren".to_string(),
                "Plugin-Einstellungen zurücksetzen".to_string(),
            ],
            _ => vec!["Anwendung neu starten".to_string()],
        }
    }
}

impl PluginErrorType {
    /// Gibt eine benutzerfreundliche Beschreibung zurück
    pub fn description(&self) -> &'static str {
        match self {
            PluginErrorType::LoadError => "Plugin konnte nicht geladen werden",
            PluginErrorType::ExecutionError => "Fehler bei der Plugin-Ausführung",
            PluginErrorType::ConfigError => "Plugin-Konfigurationsfehler",
            PluginErrorType::DependencyError => "Plugin-Abhängigkeiten fehlen",
            PluginErrorType::PermissionError => "Plugin-Berechtigungen unzureichend",
            PluginErrorType::VersionError => "Plugin-Version inkompatibel",
            PluginErrorType::ApiError => "Plugin-API-Fehler",
            PluginErrorType::Unknown => "Unbekannter Plugin-Fehler",
        }
    }

    /// Prüft ob der Plugin-Fehler kritisch ist
    pub fn is_critical(&self) -> bool {
        matches!(self, 
            PluginErrorType::LoadError | 
            PluginErrorType::DependencyError |
            PluginErrorType::VersionError
        )
    }
}

impl SecuritySeverity {
    /// Gibt eine Farbe für die UI zurück
    pub fn color(&self) -> &'static str {
        match self {
            SecuritySeverity::Low => "#28a745",      // Grün
            SecuritySeverity::Medium => "#ffc107",   // Gelb
            SecuritySeverity::High => "#fd7e14",     // Orange
            SecuritySeverity::Critical => "#dc3545", // Rot
        }
    }

    /// Gibt ein Icon für die UI zurück
    pub fn icon(&self) -> &'static str {
        match self {
            SecuritySeverity::Low => "🔒",
            SecuritySeverity::Medium => "⚠️",
            SecuritySeverity::High => "🚨",
            SecuritySeverity::Critical => "🔴",
        }
    }

    /// Prüft ob sofortige Aktion erforderlich ist
    pub fn requires_immediate_action(&self) -> bool {
        matches!(self, SecuritySeverity::High | SecuritySeverity::Critical)
    }
}

impl StorageOperation {
    /// Prüft ob die Operation schreibend ist
    pub fn is_write_operation(&self) -> bool {
        matches!(self, 
            StorageOperation::Write | 
            StorageOperation::Delete |
            StorageOperation::Initialize |
            StorageOperation::Backup |
            StorageOperation::Restore |
            StorageOperation::Sync
        )
    }

    /// Prüft ob die Operation lesend ist
    pub fn is_read_operation(&self) -> bool {
        matches!(self, StorageOperation::Read | StorageOperation::List)
    }

    /// Gibt eine benutzerfreundliche Beschreibung zurück
    pub fn description(&self) -> &'static str {
        match self {
            StorageOperation::Read => "Daten lesen",
            StorageOperation::Write => "Daten schreiben",
            StorageOperation::Delete => "Daten löschen",
            StorageOperation::List => "Daten auflisten",
            StorageOperation::Initialize => "Storage initialisieren",
            StorageOperation::Backup => "Backup erstellen",
            StorageOperation::Restore => "Daten wiederherstellen",
            StorageOperation::Sync => "Daten synchronisieren",
        }
    }
}

impl ErrorCategory {
    /// Gibt ein Icon für die Kategorie zurück
    pub fn icon(&self) -> &'static str {
        match self {
            ErrorCategory::Network => "🌐",
            ErrorCategory::FileSystem => "📁",
            ErrorCategory::UserInterface => "🖥️",
            ErrorCategory::Plugin => "🔌",
            ErrorCategory::Security => "🛡️",
            ErrorCategory::Configuration => "⚙️",
            ErrorCategory::System => "🔧",
            ErrorCategory::Unknown => "❓",
        }
    }

    /// Gibt eine Beschreibung der Kategorie zurück
    pub fn description(&self) -> &'static str {
        match self {
            ErrorCategory::Network => "Netzwerk- und Verbindungsfehler",
            ErrorCategory::FileSystem => "Dateisystem- und Storage-Fehler",
            ErrorCategory::UserInterface => "Benutzeroberflächen- und Rendering-Fehler",
            ErrorCategory::Plugin => "Plugin- und Erweiterungs-Fehler",
            ErrorCategory::Security => "Sicherheits- und Zertifikatsfehler",
            ErrorCategory::Configuration => "Konfigurations- und Einstellungsfehler",
            ErrorCategory::System => "System- und Initialisierungsfehler",
            ErrorCategory::Unknown => "Unbekannte oder nicht kategorisierte Fehler",
        }
    }
}

/// Hilfsfunktionen für Error-Erstellung
impl ZAKYXBrowserError {
    /// Erstellt einen einfachen Network-Error
    pub fn network_error(message: impl Into<String>) -> Self {
        ZAKYXBrowserError::Network {
            message: message.into(),
            url: None,
            retry_possible: true,
        }
    }

    /// Erstellt einen HTTP-Error
    pub fn http_error(status: u16, message: impl Into<String>, url: impl Into<String>) -> Self {
        ZAKYXBrowserError::Http {
            status,
            message: message.into(),
            url: url.into(),
            retry_possible: status >= 500, // Server-Errors sind retryable
        }
    }

    /// Erstellt einen Timeout-Error
    pub fn timeout_error(seconds: u64, operation: impl Into<String>) -> Self {
        ZAKYXBrowserError::Timeout {
            seconds,
            operation: operation.into(),
            retry_possible: true,
        }
    }

    /// Erstellt einen Plugin-Error
    pub fn plugin_error(plugin_id: impl Into<String>, message: impl Into<String>, error_type: PluginErrorType) -> Self {
        ZAKYXBrowserError::Plugin {
            plugin_id: plugin_id.into(),
            message: message.into(),
            error_type,
        }
    }

    /// Erstellt einen Security-Error
    pub fn security_error(message: impl Into<String>, severity: SecuritySeverity, blocked_action: impl Into<String>) -> Self {
        ZAKYXBrowserError::Security {
            message: message.into(),
            severity,
            blocked_action: blocked_action.into(),
        }
    }

    /// Erstellt einen Config-Error
    pub fn config_error(message: impl Into<String>) -> Self {
        ZAKYXBrowserError::Config {
            message: message.into(),
            field: None,
            fix_suggestion: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_category() {
        let network_error = ZAKYXBrowserError::network_error("Test error");
        assert_eq!(network_error.category(), ErrorCategory::Network);
        
        let plugin_error = ZAKYXBrowserError::plugin_error("test-plugin", "Test error", PluginErrorType::LoadError);
        assert_eq!(plugin_error.category(), ErrorCategory::Plugin);
    }

    #[test]
    fn test_error_retryability() {
        let retryable_error = ZAKYXBrowserError::network_error("Test error");
        assert!(retryable_error.is_retryable());
        
        let non_retryable_error = ZAKYXBrowserError::config_error("Invalid config");
        assert!(!non_retryable_error.is_retryable());
    }

    #[test]
    fn test_security_severity() {
        assert_eq!(SecuritySeverity::Critical.color(), "#dc3545");
        assert_eq!(SecuritySeverity::Critical.icon(), "🔴");
        assert!(SecuritySeverity::Critical.requires_immediate_action());
        
        assert!(!SecuritySeverity::Low.requires_immediate_action());
    }

    #[test]
    fn test_storage_operations() {
        assert!(StorageOperation::Write.is_write_operation());
        assert!(!StorageOperation::Write.is_read_operation());
        
        assert!(StorageOperation::Read.is_read_operation());
        assert!(!StorageOperation::Read.is_write_operation());
    }

    #[test]
    fn test_plugin_error_type() {
        assert!(PluginErrorType::LoadError.is_critical());
        assert!(!PluginErrorType::ExecutionError.is_critical());
        
        assert_eq!(PluginErrorType::LoadError.description(), "Plugin konnte nicht geladen werden");
    }

    #[test]
    fn test_user_messages() {
        let error = ZAKYXBrowserError::network_error("Connection failed");
        assert!(error.user_message().contains("Netzwerkproblem"));
        
        let actions = error.suggested_actions();
        assert!(!actions.is_empty());
        assert!(actions.iter().any(|a| a.contains("Internetverbindung")));
    }

    #[test]
    fn test_error_helpers() {
        let http_error = ZAKYXBrowserError::http_error(404, "Not Found", "https://example.com");
        if let ZAKYXBrowserError::Http { status, message, url, retry_possible } = http_error {
            assert_eq!(status, 404);
            assert_eq!(message, "Not Found");
            assert_eq!(url, "https://example.com");
            assert!(!retry_possible); // 404 ist nicht retryable
        } else {
            panic!("Expected Http error");
        }
    }
}