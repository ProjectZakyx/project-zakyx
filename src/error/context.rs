// 🔍 ERROR CONTEXT
// Erweiterte Debugging-Informationen für Fehler
// Copyright © 2024 ZAKYX Browser Team

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use crate::error::ZAKYXBrowserError;

/// Error-Context für detaillierte Fehlerdiagnose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub error_id: String,
    pub timestamp: u64,
    pub module: String,
    pub function: String,
    pub line: Option<u32>,
    pub file: Option<String>,
    pub thread_id: Option<String>,
    pub user_action: Option<String>,
    pub url_context: Option<String>,
    pub plugin_context: Option<String>,
    pub custom_data: HashMap<String, String>,
    pub stack_trace: Vec<String>,
    pub system_info: SystemInfo,
}

/// System-Informationen für Error-Reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub browser_version: String,
    pub rust_version: String,
    pub memory_usage: Option<u64>,
    pub cpu_usage: Option<f32>,
}

impl ErrorContext {
    /// Erstelle neuen Error-Context mit Modul und Funktion
    pub fn new(module: &str, function: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let error_id = format!("ERR-{}-{}", timestamp, rand::random::<u32>());
        
        Self {
            error_id,
            timestamp,
            module: module.to_string(),
            function: function.to_string(),
            line: None,
            file: None,
            thread_id: std::thread::current().name().map(|s| s.to_string()),
            user_action: None,
            url_context: None,
            plugin_context: None,
            custom_data: HashMap::new(),
            stack_trace: Vec::new(),
            system_info: SystemInfo::current(),
        }
    }
    
    /// Füge Code-Location hinzu
    pub fn with_location(mut self, file: &str, line: u32) -> Self {
        self.file = Some(file.to_string());
        self.line = Some(line);
        self
    }
    
    /// Füge User-Action-Context hinzu
    pub fn with_user_action(mut self, action: &str) -> Self {
        self.user_action = Some(action.to_string());
        self
    }
    
    /// Füge URL-Context hinzu
    pub fn with_url(mut self, url: &str) -> Self {
        self.url_context = Some(url.to_string());
        self
    }
    
    /// Füge Plugin-Context hinzu
    pub fn with_plugin(mut self, plugin_id: &str) -> Self {
        self.plugin_context = Some(plugin_id.to_string());
        self
    }
    
    /// Füge Custom-Data hinzu
    pub fn with_data(mut self, key: &str, value: &str) -> Self {
        self.custom_data.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Füge Stack-Trace hinzu
    pub fn with_stack_trace(mut self, trace: Vec<String>) -> Self {
        self.stack_trace = trace;
        self
    }
    
    /// Erstelle Error-Report
    pub fn to_report(&self) -> String {
        format!(
            "🚨 ERROR REPORT [{}]\n\
            📅 Timestamp: {}\n\
            📍 Location: {}::{}\n\
            📁 File: {}:{}\n\
            🧵 Thread: {}\n\
            👤 User Action: {}\n\
            🌐 URL: {}\n\
            🔌 Plugin: {}\n\
            💻 System: {} {}\n\
            📊 Custom Data: {}\n\
            📋 Stack Trace:\n{}",
            self.error_id,
            self.timestamp,
            self.module,
            self.function,
            self.file.as_deref().unwrap_or("unknown"),
            self.line.unwrap_or(0),
            self.thread_id.as_deref().unwrap_or("unknown"),
            self.user_action.as_deref().unwrap_or("none"),
            self.url_context.as_deref().unwrap_or("none"),
            self.plugin_context.as_deref().unwrap_or("none"),
            self.system_info.os,
            self.system_info.arch,
            serde_json::to_string_pretty(&self.custom_data).unwrap_or_default(),
            self.stack_trace.join("\n  ")
        )
    }
    
    /// Exportiere als JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl SystemInfo {
    /// Hole aktuelle System-Informationen
    pub fn current() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            browser_version: env!("CARGO_PKG_VERSION").to_string(),
            rust_version: "unknown".to_string(), // TODO: Add build script for RUSTC_VERSION
            memory_usage: None, // TODO: Implementiere Memory-Monitoring
            cpu_usage: None,    // TODO: Implementiere CPU-Monitoring
        }
    }
}

/// Macro für einfache Error-Context-Erstellung
#[macro_export]
macro_rules! error_context {
    ($module:expr, $function:expr) => {
        crate::error::context::ErrorContext::new($module, $function)
            .with_location(file!(), line!())
    };
    
    ($module:expr, $function:expr, $($key:expr => $value:expr),+) => {
        {
            let mut ctx = crate::error::context::ErrorContext::new($module, $function)
                .with_location(file!(), line!());
            $(
                ctx = ctx.with_data($key, $value);
            )+
            ctx
        }
    };
}

/// Erweiterte Error-Wrapper mit Context
#[derive(Debug, Clone)]
pub struct ContextualError {
    pub error: ZAKYXBrowserError,
    pub context: ErrorContext,
}

impl ContextualError {
    /// Erstelle neuen Contextual Error
    pub fn new(error: ZAKYXBrowserError, context: ErrorContext) -> Self {
        Self { error, context }
    }
    
    /// Hole die Error-Message mit Context
    pub fn full_message(&self) -> String {
        format!(
            "{}\n\nContext:\n{}",
            self.error.user_message(),
            self.context.to_report()
        )
    }
    
    /// Log den Fehler mit allen Details
    pub fn log(&self) {
        match self.error.log_level() {
            crate::error::LogLevel::Error => {
                tracing::error!("🚨 {}", self.full_message());
            },
            crate::error::LogLevel::Warn => {
                tracing::warn!("⚠️ {}", self.full_message());
            },
            crate::error::LogLevel::Info => {
                tracing::info!("ℹ️ {}", self.full_message());
            },
            crate::error::LogLevel::Debug => {
                tracing::debug!("🔍 {}", self.full_message());
            },
        }
    }
}

impl std::fmt::Display for ContextualError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_message())
    }
}

impl std::error::Error for ContextualError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// Result-Type mit Context
pub type ContextualResult<T> = Result<T, ContextualError>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_context_creation() {
        let ctx = ErrorContext::new("test_module", "test_function")
            .with_location("test.rs", 42)
            .with_user_action("clicking button")
            .with_url("https://example.com")
            .with_plugin("test-plugin")
            .with_data("retry_count", "3")
            .with_data("timeout", "30s");
        
        assert_eq!(ctx.module, "test_module");
        assert_eq!(ctx.function, "test_function");
        assert_eq!(ctx.file, Some("test.rs".to_string()));
        assert_eq!(ctx.line, Some(42));
        assert_eq!(ctx.user_action, Some("clicking button".to_string()));
        assert_eq!(ctx.url_context, Some("https://example.com".to_string()));
        assert_eq!(ctx.plugin_context, Some("test-plugin".to_string()));
        assert_eq!(ctx.custom_data.get("retry_count"), Some(&"3".to_string()));
        assert_eq!(ctx.custom_data.get("timeout"), Some(&"30s".to_string()));
    }
    
    #[test]
    fn test_error_context_macro() {
        let ctx = error_context!("test_module", "test_function");
        assert_eq!(ctx.module, "test_module");
        assert_eq!(ctx.function, "test_function");
        assert!(ctx.file.is_some());
        assert!(ctx.line.is_some());
    }
    
    #[test]
    fn test_error_context_macro_with_data() {
        let ctx = error_context!(
            "test_module", 
            "test_function",
            "url" => "https://example.com",
            "plugin" => "test-plugin"
        );
        
        assert_eq!(ctx.custom_data.get("url"), Some(&"https://example.com".to_string()));
        assert_eq!(ctx.custom_data.get("plugin"), Some(&"test-plugin".to_string()));
    }
    
    #[test]
    fn test_contextual_error() {
        let error = ZAKYXBrowserError::Plugin {
            plugin_id: "test-plugin".to_string(),
            message: "Test error".to_string(),
        };
        
        let context = ErrorContext::new("test_module", "test_function");
        let contextual = ContextualError::new(error, context);
        
        let message = contextual.full_message();
        assert!(message.contains("Test error"));
        assert!(message.contains("test_module"));
        assert!(message.contains("test_function"));
    }
} 
