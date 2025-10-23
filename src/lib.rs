//! # ZAKYX Browser Library
//!
//! ZAKYX Browser ist ein moderner, sicherheitsorientierter Web-Browser
//! mit erweiterten Features und plattformübergreifender Unterstützung.

#![allow(dead_code)]
#![allow(unused_imports)]

// Core modules - nur existierende Module
pub mod browser;
pub mod ui;
pub mod platform; // Neues modulares Platform-System
pub mod webview2; // Neues modulares WebView2-System
// pub mod real_webview2; // Deaktiviert - verwende real_webview2_engine.rs
pub mod proxy_server;
pub mod proxy;
pub mod error;
pub mod error_system; // Neues modulares Error-System
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
pub use platform::{Platform, PlatformExtensionManager, CrossPlatformManager}; // Platform-System
pub use webview2::{OptimizedWebView2, OptimizedWebView2Manager, WebView2Config}; // WebView2-System
// pub use real_webview2::{RealWebView2Engine, RealWebView2Config, WebView2Container}; // Real WebView2-System
pub use proxy::SmartProxy;
pub use proxy_server::ProxyServer;
pub use error::{ZAKYXBrowserError, ZAKYXBrowserResult};
pub use error_system::{ZAKYXErrorSystem, ErrorRecoveryManager, ErrorHandlers}; // Error-System
pub use internal_webview2_navigation::{InternalWebView2Navigator, WebViewConfig};
pub use browser_state::{BrowserState, Tab, Bookmark, BrowserSettings};
pub use plugin::{PluginManager, PluginInfo, PluginManifest};
pub use config::ZAKYXConfig;
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
