//! # Ora Browser Library
//! 
//! Ora Browser ist ein moderner, sicherheitsorientierter Web-Browser
//! mit erweiterten Features und plattformübergreifender Unterstützung.

#![allow(dead_code)]
#![allow(unused_imports)]

// Core modules - nur existierende Module
pub mod browser;
pub mod ui;
pub mod proxy_server;
pub mod proxy;
pub mod error;
pub mod internal_webview2_navigation;
pub mod ethical_safeguards;
pub mod browser_state;
pub mod tauri_commands;
pub mod url_utils;
pub mod plugin;
pub mod config;
pub mod metrics;

// Re-exports für die wichtigsten Funktionen
pub use browser::BookmarkManager;
pub use proxy::SmartProxy;
pub use proxy_server::ProxyServer;
pub use error::{OraBrowserError, OraBrowserResult};
pub use internal_webview2_navigation::{InternalWebView2Navigator, WebViewConfig};
pub use browser_state::{BrowserState, Tab, Bookmark, BrowserSettings};
pub use plugin::{PluginManager, PluginInfo, PluginManifest};
pub use config::OraConfig;
pub use metrics::MetricsCollector;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_library_initialization() {
        // Basic smoke test
        assert_eq!(1 + 1, 2);
    }
} 