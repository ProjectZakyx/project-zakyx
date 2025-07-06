// 🛠️ ERROR HELPERS
// Utility-Funktionen für Error-Handling
// Copyright © 2024 Ora Browser Team

use crate::error::{OraBrowserError, OraBrowserResult};
use crate::error::context::{ErrorContext, ContextualError, ContextualResult};

/// Helper-Trait für einfache Error-Konversion
pub trait ErrorHelpers<T> {
    /// Konvertiere String-Error zu OraBrowserError
    fn to_ora_error(self) -> OraBrowserResult<T>;
    
    /// Konvertiere mit Context
    fn with_context(self, context: ErrorContext) -> ContextualResult<T>;
    
    /// Konvertiere Plugin-Error
    fn plugin_error(self, plugin_id: &str) -> OraBrowserResult<T>;
    
    /// Konvertiere Network-Error
    fn network_error(self, url: Option<&str>) -> OraBrowserResult<T>;
    
    /// Konvertiere Config-Error
    fn config_error(self, field: Option<&str>) -> OraBrowserResult<T>;
    
    /// Konvertiere UI-Error
    fn ui_error(self, component: &str) -> OraBrowserResult<T>;
}

impl<T> ErrorHelpers<T> for Result<T, String> {
    fn to_ora_error(self) -> OraBrowserResult<T> {
        self.map_err(|e| OraBrowserError::Unknown { message: e })
    }
    
    fn with_context(self, context: ErrorContext) -> ContextualResult<T> {
        match self {
            Ok(val) => Ok(val),
            Err(msg) => {
                let error = OraBrowserError::Unknown { message: msg };
                Err(ContextualError::new(error, context))
            }
        }
    }
    
    fn plugin_error(self, plugin_id: &str) -> OraBrowserResult<T> {
        self.map_err(|msg| OraBrowserError::Plugin {
            plugin_id: plugin_id.to_string(),
            message: msg,
        })
    }
    
    fn network_error(self, url: Option<&str>) -> OraBrowserResult<T> {
        self.map_err(|msg| OraBrowserError::Network {
            message: msg,
            url: url.map(|s| s.to_string()),
            retry_possible: true,
        })
    }
    
    fn config_error(self, field: Option<&str>) -> OraBrowserResult<T> {
        self.map_err(|msg| OraBrowserError::Config {
            message: msg,
            field: field.map(|s| s.to_string()),
            fix_suggestion: None,
        })
    }
    
    fn ui_error(self, component: &str) -> OraBrowserResult<T> {
        self.map_err(|msg| OraBrowserError::Ui {
            message: msg,
            component: component.to_string(),
            recoverable: true,
        })
    }
}

/// Macros für häufige Error-Erstellung
#[macro_export]
macro_rules! plugin_error {
    ($plugin_id:expr, $msg:expr) => {
        crate::error::OraBrowserError::Plugin {
            plugin_id: $plugin_id.to_string(),
            message: $msg.to_string(),
        }
    };
}

#[macro_export]
macro_rules! network_error {
    ($msg:expr) => {
        crate::error::OraBrowserError::Network {
            message: $msg.to_string(),
            url: None,
            retry_possible: true,
        }
    };
    ($msg:expr, $url:expr) => {
        crate::error::OraBrowserError::Network {
            message: $msg.to_string(),
            url: Some($url.to_string()),
            retry_possible: true,
        }
    };
}

#[macro_export]
macro_rules! config_error {
    ($msg:expr) => {
        crate::error::OraBrowserError::Config {
            message: $msg.to_string(),
            field: None,
            fix_suggestion: None,
        }
    };
    ($msg:expr, $field:expr) => {
        crate::error::OraBrowserError::Config {
            message: $msg.to_string(),
            field: Some($field.to_string()),
            fix_suggestion: None,
        }
    };
    ($msg:expr, $field:expr, $fix:expr) => {
        crate::error::OraBrowserError::Config {
            message: $msg.to_string(),
            field: Some($field.to_string()),
            fix_suggestion: Some($fix.to_string()),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::recovery::ErrorRecovery;
    use std::cell::Cell;
    
    #[test]
    fn test_string_error_conversion() {
        let result: Result<(), String> = Err("test error".to_string());
        let ora_result = result.to_ora_error();
        
        assert!(ora_result.is_err());
        match ora_result.unwrap_err() {
            OraBrowserError::Unknown { message } => assert_eq!(message, "test error"),
            _ => panic!("Wrong error type"),
        }
    }
    
    #[test]
    fn test_plugin_error_conversion() {
        let result: Result<(), String> = Err("plugin failed".to_string());
        let ora_result = result.plugin_error("test-plugin");
        
        assert!(ora_result.is_err());
        match ora_result.unwrap_err() {
            OraBrowserError::Plugin { plugin_id, message } => {
                assert_eq!(plugin_id, "test-plugin");
                assert_eq!(message, "plugin failed");
            },
            _ => panic!("Wrong error type"),
        }
    }
    
    #[test]
    fn test_error_macros() {
        let error = plugin_error!("test-plugin", "test message");
        match error {
            OraBrowserError::Plugin { plugin_id, message } => {
                assert_eq!(plugin_id, "test-plugin");
                assert_eq!(message, "test message");
            },
            _ => panic!("Wrong error type"),
        }
        
        let error = network_error!("connection failed");
        match error {
            OraBrowserError::Network { message, url, retry_possible } => {
                assert_eq!(message, "connection failed");
                assert_eq!(url, None);
                assert!(retry_possible);
            },
            _ => panic!("Wrong error type"),
        }
        
        let error = network_error!("connection failed", "https://example.com");
        match error {
            OraBrowserError::Network { message, url, retry_possible } => {
                assert_eq!(message, "connection failed");
                assert_eq!(url, Some("https://example.com".to_string()));
                assert!(retry_possible);
            },
            _ => panic!("Wrong error type"),
        }
    }
    
    #[tokio::test]
    async fn test_error_recovery_with_retry() {
        let attempt_count = Cell::new(0);
        
        let result = ErrorRecovery::with_retry(
            || {
                let count = attempt_count.get() + 1;
                attempt_count.set(count);
                async move {
                    if count < 3 {
                        Err(OraBrowserError::Network {
                            message: "temporary failure".to_string(),
                            url: None,
                            retry_possible: true,
                        })
                    } else {
                        Ok("success")
                    }
                }
            },
            3,
            10, // 10ms delay for fast test
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempt_count.get(), 3);
    }
} 