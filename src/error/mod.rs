// 🚨 ERROR MODULE
// Modulares Error-System für ZAKYX Browser
// Copyright © 2024 ZAKYX Browser Team

#![allow(dead_code)] // Error system - helper functions kept for API completeness

pub mod types;
pub mod context;
pub mod helpers;
pub mod recovery;

// Re-export everything from types
pub use types::*;

// Re-export from submodules
pub use context::ErrorContext;

// Extended error types for specific modules
impl ZAKYXBrowserError {
    /// Erstelle Plugin-spezifischen Error
    pub fn plugin_error(plugin_id: &str, message: &str) -> Self {
        Self::Plugin {
            plugin_id: plugin_id.to_string(),
            message: message.to_string(),
        }
    }
    
    /// Erstelle Network-Error mit URL
    pub fn network_error(message: &str, url: Option<&str>) -> Self {
        Self::Network {
            message: message.to_string(),
            url: url.map(|s| s.to_string()),
            retry_possible: true,
        }
    }
    
    /// Erstelle Config-Error mit Suggestion
    pub fn config_error(message: &str, field: Option<&str>, fix: Option<&str>) -> Self {
        Self::Config {
            message: message.to_string(),
            field: field.map(|s| s.to_string()),
            fix_suggestion: fix.map(|s| s.to_string()),
        }
    }
    
    /// Erstelle UI-Error
    pub fn ui_error(component: &str, message: &str, recoverable: bool) -> Self {
        Self::Ui {
            component: component.to_string(),
            message: message.to_string(),
            recoverable,
        }
    }
    
    /// Erstelle Proxy-Error
    pub fn proxy_error(message: &str, url: Option<&str>) -> Self {
        Self::Proxy {
            message: message.to_string(),
            url: url.map(|s| s.to_string()),
        }
    }
    
    /// Erstelle Security-Error
    pub fn security_error(message: &str) -> Self {
        Self::Security {
            message: message.to_string(),
        }
    }
    
    /// Erstelle Timeout-Error
    pub fn timeout_error(operation: &str, seconds: u64) -> Self {
        Self::Timeout {
            operation: operation.to_string(),
            seconds,
            retry_possible: true,
        }
    }
    
    /// Erstelle Storage-Error
    pub fn storage_error(operation: &str, message: &str, key: Option<&str>) -> Self {
        use crate::error::StorageOperation;
        
        let storage_op = match operation {
            "read" => StorageOperation::Read,
            "write" => StorageOperation::Write,
            "delete" => StorageOperation::Delete,
            "list" => StorageOperation::List,
            _ => StorageOperation::Initialize,
        };
        
        Self::Storage {
            message: message.to_string(),
            operation: storage_op,
            key: key.map(|s| s.to_string()),
        }
    }
    
    /// Prüfe, ob der Error einen bestimmten Typ hat
    pub fn is_plugin_error(&self) -> bool {
        matches!(self, Self::Plugin { .. } | Self::PluginNotFound { .. } | Self::PluginAlreadyLoaded { .. })
    }
    
    pub fn is_network_error(&self) -> bool {
        matches!(self, Self::Network { .. } | Self::Http { .. } | Self::Timeout { .. })
    }
    
    pub fn is_security_error(&self) -> bool {
        matches!(self, Self::Security { .. } | Self::Cors { .. } | Self::Csp { .. })
    }
    
    pub fn is_config_error(&self) -> bool {
        matches!(self, Self::Config { .. } | Self::InvalidUrl { .. } | Self::FileNotFound { .. })
    }
    
    /// Hole die Plugin-ID falls verfügbar
    pub fn plugin_id(&self) -> Option<&str> {
        match self {
            Self::Plugin { plugin_id, .. } | 
            Self::PluginNotFound { plugin_id } | 
            Self::PluginAlreadyLoaded { plugin_id } |
            Self::InvalidPluginPermission { plugin_id, .. } => Some(plugin_id),
            _ => None,
        }
    }
    
    /// Hole die URL falls verfügbar
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Network { url, .. } => url.as_deref(),
            Self::Http { url, .. } => Some(url),
            Self::Cors { url, .. } => Some(url),
            Self::Csp { url, .. } => Some(url),
            Self::Proxy { url, .. } => url.as_deref(),
            Self::WebView { url, .. } => url.as_deref(),
            Self::ContentSizeLimit { url, .. } => Some(url),
            Self::InvalidUrl { url, .. } => Some(url),
            _ => None,
        }
    }
    
    /// Erstelle einen Error-Report
    pub fn to_report(&self, context: Option<&ErrorContext>) -> String {
        let base_message = self.user_message();
        let suggested_actions = self.suggested_actions();
        let recovery_strategy = self.recovery_strategy();
        
        let mut report = format!(
            "🚨 ERROR REPORT\n\
            📋 Message: {}\n\
            🔄 Retryable: {}\n\
            📊 Log Level: {:?}\n\
            🛠️ Suggested Actions:\n{}",
            base_message,
            self.is_retryable(),
            self.log_level(),
            suggested_actions.iter().map(|a| format!("  • {}", a)).collect::<Vec<_>>().join("\n")
        );
        
        if recovery_strategy.retry_possible {
            report.push_str(&format!(
                "\n🔄 Recovery Strategy:\n\
                  • Max Retries: {}\n\
                  • Retry Delay: {}ms\n\
                  • Fallback Available: {}\n\
                  • User Action Required: {}",
                recovery_strategy.max_retries,
                recovery_strategy.retry_delay_ms,
                recovery_strategy.fallback_available,
                recovery_strategy.user_action_required
            ));
        }
        
        if let Some(ctx) = context {
            report.push_str(&format!("\n\n{}", ctx.to_report()));
        }
        
        report
    }
}

/// Standard Error-Konvertierungen für häufige externe Types
impl From<Box<dyn std::error::Error + Send + Sync>> for ZAKYXBrowserError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Unknown {
            message: err.to_string(),
        }
    }
}

impl From<anyhow::Error> for ZAKYXBrowserError {
    fn from(err: anyhow::Error) -> Self {
        Self::Unknown {
            message: err.to_string(),
        }
    }
}

/// Convenience-Funktionen für häufige Patterns
pub fn plugin_not_found(plugin_id: &str) -> ZAKYXBrowserError {
    ZAKYXBrowserError::PluginNotFound {
        plugin_id: plugin_id.to_string(),
    }
}

pub fn plugin_already_loaded(plugin_id: &str) -> ZAKYXBrowserError {
    ZAKYXBrowserError::PluginAlreadyLoaded {
        plugin_id: plugin_id.to_string(),
    }
}

pub fn invalid_url(url: &str, reason: &str) -> ZAKYXBrowserError {
    ZAKYXBrowserError::InvalidUrl {
        url: url.to_string(),
        reason: reason.to_string(),
    }
}

pub fn file_not_found(path: &str, operation: &str) -> ZAKYXBrowserError {
    ZAKYXBrowserError::FileNotFound {
        path: path.to_string(),
        operation: operation.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_type_checks() {
        let plugin_error = ZAKYXBrowserError::plugin_error("test-plugin", "test message");
        assert!(plugin_error.is_plugin_error());
        assert!(!plugin_error.is_network_error());
        assert_eq!(plugin_error.plugin_id(), Some("test-plugin"));
        
        let network_error = ZAKYXBrowserError::network_error("connection failed", Some("https://example.com"));
        assert!(network_error.is_network_error());
        assert!(!network_error.is_plugin_error());
        assert_eq!(network_error.url(), Some("https://example.com"));
    }
    
    #[test]
    fn test_convenience_functions() {
        let error = plugin_not_found("missing-plugin");
        match error {
            ZAKYXBrowserError::PluginNotFound { plugin_id } => {
                assert_eq!(plugin_id, "missing-plugin");
            },
            _ => panic!("Wrong error type"),
        }
        
        let error = invalid_url("not-a-url", "missing protocol");
        match error {
            ZAKYXBrowserError::InvalidUrl { url, reason } => {
                assert_eq!(url, "not-a-url");
                assert_eq!(reason, "missing protocol");
            },
            _ => panic!("Wrong error type"),
        }
    }
    
    #[test]
    fn test_error_report() {
        let error = ZAKYXBrowserError::plugin_error("test-plugin", "test message");
        let report = error.to_report(None);
        
        assert!(report.contains("ERROR REPORT"));
        assert!(report.contains("test message"));
        assert!(report.contains("Suggested Actions"));
    }
} 
