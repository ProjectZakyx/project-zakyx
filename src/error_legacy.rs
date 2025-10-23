// 🚨 ZAKYX BROWSER ERROR LEGACY - LEGACY FILE
// Diese Datei bleibt für Rückwärtskompatibilität
// Verwende stattdessen src/error_system/ für neue Entwicklungen

// Re-export der neuen modularen Struktur
pub use crate::error_system::{
    ZAKYXBrowserError,
    PluginErrorType,
    SecuritySeverity,
    StorageOperation,
    ErrorCategory,
    ErrorRecovery,
    ErrorRecoveryManager,
    RecoveryStrategy,
    ErrorHandlers,
    ErrorContext,
    ZAKYXErrorSystem,
    ZAKYXBrowserResult,
    handle_global_error,
    initialize_global_error_system,
};

/// Legacy Error-Type (deprecated - use crate::error_system::ZAKYXBrowserError)
#[deprecated(note = "Use crate::error_system::ZAKYXBrowserError instead")]
pub type LegacyZAKYXBrowserError = ZAKYXBrowserError;

/// Legacy Result-Type (deprecated - use crate::error_system::ZAKYXBrowserResult)
#[deprecated(note = "Use crate::error_system::ZAKYXBrowserResult instead")]
pub type ZAKYXBrowserResult<T> = Result<T, ZAKYXBrowserError>;
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

    #[error("Datei nicht gefunden: {path}")]
    FileNotFound { 
        path: String,
        operation: String,
    },

    #[error("Berechtigungsfehler: {message}")]
    Permission { 
        message: String,
        required_permission: String,
    },

    // 🔌 PLUGIN ERRORS
    #[error("Plugin-Fehler '{plugin_id}': {message}")]
    Plugin { 
        plugin_id: String,
        message: String,
    },

    #[error("Plugin '{plugin_id}' nicht gefunden")]
    PluginNotFound { 
        plugin_id: String,
    },

    #[error("Plugin '{plugin_id}' bereits geladen")]
    PluginAlreadyLoaded { 
        plugin_id: String,
    },

    #[error("Ungültige Plugin-Berechtigung: {permission}")]
    InvalidPluginPermission { 
        plugin_id: String,
        permission: String,
    },

    // 🛡️ SECURITY ERRORS
    #[error("Sicherheitsfehler: {message}")]
    Security { 
        message: String,
    },

    #[error("CORS-Fehler für {url}: {message}")]
    Cors { 
        url: String,
        message: String,
    },

    #[error("Content Security Policy Verletzung: {violation}")]
    Csp { 
        violation: String,
        url: String,
    },

    // 📱 UI & FRONTEND ERRORS
    #[error("UI-Fehler: {message}")]
    Ui { 
        message: String,
        component: String,
        recoverable: bool,
    },

    #[error("WebView-Fehler: {message}")]
    WebView { 
        message: String,
        url: Option<String>,
    },

    #[error("JavaScript-Fehler: {message}")]
    JavaScript { 
        message: String,
        file: Option<String>,
        line: Option<u32>,
    },

    // 🗄️ STORAGE & DATA ERRORS
    #[error("Speicherfehler: {message}")]
    Storage { 
        message: String,
        operation: StorageOperation,
        key: Option<String>,
    },

    #[error("Serialisierungsfehler: {message}")]
    Serialization { 
        message: String,
        data_type: String,
    },

    #[error("Datenbankfehler: {message}")]
    Database { 
        message: String,
        operation: String,
    },

    // 🔄 PROXY ERRORS
    #[error("Proxy-Fehler: {message}")]
    Proxy { 
        message: String,
        url: Option<String>,
    },

    // 🚫 GENERIC ERRORS
    #[error("Unbekannter Fehler: {message}")]
    Unknown { 
        message: String,
    },

    #[error("Funktion nicht implementiert: {feature}")]
    NotImplemented { 
        feature: String,
        planned_version: Option<String>,
    },

    #[error("Interner Fehler: {message}")]
    Internal { 
        message: String,
        module: String,
    },
}

/// Plugin-spezifische Error-Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginErrorType {
    LoadingFailed,
    ExecutionFailed,
    ConfigurationError,
    PermissionDenied,
    DependencyMissing,
    VersionIncompatible,
}

/// Sicherheitsbedrohungs-Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityThreatType {
    XssAttempt,
    CsrfAttempt,
    MaliciousScript,
    UnsafeRedirect,
    SuspiciousContent,
    UnauthorizedAccess,
}

/// Sicherheitsschweregrad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Storage-Operationen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageOperation {
    Read,
    Write,
    Delete,
    List,
    Initialize,
}

/// Error-Recovery-Strategien
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRecovery {
    pub retry_possible: bool,
    pub retry_count: u32,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub fallback_available: bool,
    pub user_action_required: bool,
}

impl ZAKYXBrowserError {
    /// Prüfe ob der Fehler wiederholbar ist
    pub fn is_retryable(&self) -> bool {
        match self {
            ZAKYXBrowserError::Network { retry_possible, .. } => *retry_possible,
            ZAKYXBrowserError::Http { retry_possible, .. } => *retry_possible,
            ZAKYXBrowserError::Timeout { retry_possible, .. } => *retry_possible,
            ZAKYXBrowserError::Ui { recoverable, .. } => *recoverable,
            ZAKYXBrowserError::WebView { .. } => true, // Meistens retryable
            ZAKYXBrowserError::Proxy { .. } => true, // Proxy kann verschiedene Strategien versuchen
            _ => false,
        }
    }

    /// Erstelle Recovery-Strategie für den Fehler
    pub fn recovery_strategy(&self) -> ErrorRecovery {
        match self {
            ZAKYXBrowserError::Network { .. } => ErrorRecovery {
                retry_possible: true,
                retry_count: 0,
                max_retries: 3,
                retry_delay_ms: 1000,
                fallback_available: true,
                user_action_required: false,
            },
            ZAKYXBrowserError::Http { status, .. } => ErrorRecovery {
                retry_possible: *status >= 500, // Server-Fehler sind retryable
                retry_count: 0,
                max_retries: if *status >= 500 { 3 } else { 1 },
                retry_delay_ms: 2000,
                fallback_available: true,
                user_action_required: *status == 401, // Auth erforderlich
            },
            ZAKYXBrowserError::Timeout { .. } => ErrorRecovery {
                retry_possible: true,
                retry_count: 0,
                max_retries: 2,
                retry_delay_ms: 5000,
                fallback_available: true,
                user_action_required: false,
            },
            ZAKYXBrowserError::Plugin { .. } => ErrorRecovery {
                retry_possible: true,
                retry_count: 0,
                max_retries: 1,
                retry_delay_ms: 500,
                fallback_available: false,
                user_action_required: true,
            },
            ZAKYXBrowserError::Security { .. } => ErrorRecovery {
                retry_possible: false,
                retry_count: 0,
                max_retries: 0,
                retry_delay_ms: 0,
                fallback_available: false,
                user_action_required: true,
            },
            _ => ErrorRecovery {
                retry_possible: false,
                retry_count: 0,
                max_retries: 0,
                retry_delay_ms: 0,
                fallback_available: false,
                user_action_required: true,
            },
        }
    }

    /// Benutzerfreundliche Fehlermeldung
    pub fn user_message(&self) -> String {
        match self {
            ZAKYXBrowserError::Network { message, url, .. } => {
                match url {
                    Some(u) => format!("🌐 Verbindungsproblem mit {}: {}", u, message),
                    None => format!("🌐 Netzwerkproblem: {}", message),
                }
            },
            ZAKYXBrowserError::Http { status, url, .. } => {
                match *status {
                    404 => format!("📄 Seite nicht gefunden: {}", url),
                    403 => format!("🚫 Zugriff verweigert: {}", url),
                    500..=599 => format!("⚠️ Server-Problem bei {}", url),
                    _ => format!("🌐 HTTP-Fehler {}: {}", status, url),
                }
            },
            ZAKYXBrowserError::Timeout { operation, seconds, .. } => {
                format!("⏱️ Zeitüberschreitung bei {}: {} Sekunden", operation, seconds)
            },
            ZAKYXBrowserError::Plugin { plugin_id, message, .. } => {
                format!("🔌 Plugin '{}' Fehler: {}", plugin_id, message)
            },
            ZAKYXBrowserError::Security { message, .. } => {
                format!("🚨 Sicherheitswarnung: {}", message)
            },
            ZAKYXBrowserError::Ui { message, component, .. } => {
                format!("🖥️ Problem mit {}: {}", component, message)
            },
            ZAKYXBrowserError::Config { message, fix_suggestion, .. } => {
                match fix_suggestion {
                    Some(fix) => format!("⚙️ Konfigurationsproblem: {}. Lösung: {}", message, fix),
                    None => format!("⚙️ Konfigurationsproblem: {}", message),
                }
            },
            _ => format!("❌ {}", self),
        }
    }

    /// Gebe Lösungsvorschläge zurück
    pub fn suggested_actions(&self) -> Vec<String> {
        match self {
            ZAKYXBrowserError::Network { .. } => vec![
                "🔄 Erneut versuchen".to_string(),
                "🌐 Internetverbindung prüfen".to_string(),
                "🔍 Alternative URL verwenden".to_string(),
            ],
            ZAKYXBrowserError::Http { status, .. } => {
                match *status {
                    404 => vec!["🔍 URL auf Tippfehler prüfen".to_string(), "🏠 Zur Startseite".to_string()],
                    403 => vec!["🔐 Anmeldung erforderlich".to_string(), "📧 Administrator kontaktieren".to_string()],
                    500..=599 => vec!["⏱️ Später erneut versuchen".to_string(), "📞 Support kontaktieren".to_string()],
                    _ => vec!["🔄 Seite neu laden".to_string()],
                }
            },
            ZAKYXBrowserError::Plugin { .. } => vec![
                "🔄 Plugin neu laden".to_string(),
                "⚙️ Plugin-Einstellungen prüfen".to_string(),
                "🚫 Plugin deaktivieren".to_string(),
            ],
            ZAKYXBrowserError::Security { .. } => vec![
                "🚫 Navigation abbrechen".to_string(),
                "🔍 URL prüfen".to_string(),
                "📧 Sicherheitsteam melden".to_string(),
            ],
            _ => vec!["🔄 Erneut versuchen".to_string()],
        }
    }

    /// Log-Level für den Fehler
    pub fn log_level(&self) -> LogLevel {
        match self {
            ZAKYXBrowserError::Security { .. } => LogLevel::Error,
            ZAKYXBrowserError::Http { status, .. } => {
                match *status {
                    500..=599 => LogLevel::Error,
                    400..=499 => LogLevel::Warn,
                    _ => LogLevel::Info,
                }
            },
            ZAKYXBrowserError::Plugin { .. } | 
            ZAKYXBrowserError::Internal { .. } => LogLevel::Error,
            ZAKYXBrowserError::Config { .. } |
            ZAKYXBrowserError::Permission { .. } => LogLevel::Warn,
            _ => LogLevel::Info,
        }
    }
}

/// Log-Level für Error-Reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

/// Result-Type für ZAKYX Browser
pub type ZAKYXBrowserResult<T> = Result<T, ZAKYXBrowserError>;

// Konversionen von anderen Error-Types
impl From<reqwest::Error> for ZAKYXBrowserError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            ZAKYXBrowserError::Timeout {
                seconds: 30,
                operation: "HTTP Request".to_string(),
                retry_possible: true,
            }
        } else {
            ZAKYXBrowserError::Network {
                message: err.to_string(),
                url: err.url().map(|u| u.to_string()),
                retry_possible: true,
            }
        }
    }
}

impl From<std::io::Error> for ZAKYXBrowserError {
    fn from(err: std::io::Error) -> Self {
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
            _ => ZAKYXBrowserError::Internal {
                message: err.to_string(),
                module: "I/O".to_string(),
            },
        }
    }
}

impl From<serde_json::Error> for ZAKYXBrowserError {
    fn from(err: serde_json::Error) -> Self {
        ZAKYXBrowserError::Serialization {
            message: err.to_string(),
            data_type: "JSON".to_string(),
        }
    }
}

impl From<toml::de::Error> for ZAKYXBrowserError {
    fn from(err: toml::de::Error) -> Self {
        ZAKYXBrowserError::Serialization {
            message: err.to_string(),
            data_type: "TOML".to_string(),
        }
    }
}

// Für String-Errors (Legacy)
impl From<String> for ZAKYXBrowserError {
    fn from(err: String) -> Self {
        ZAKYXBrowserError::Unknown {
            message: err,
        }
    }
}

impl From<&str> for ZAKYXBrowserError {
    fn from(err: &str) -> Self {
        ZAKYXBrowserError::Unknown {
            message: err.to_string(),
        }
    }
} 
