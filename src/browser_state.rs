// 🗂️ BROWSER STATE & DATA STRUCTURES

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use crate::internal_webview2_navigation::InternalWebView2Navigator;
use crate::browser_features::BookmarkManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSettings {
    pub homepage: String,
    pub search_engine: String,
    pub enable_javascript: bool,
    pub enable_cookies: bool,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            homepage: "https://google.com".to_string(),
            search_engine: "https://google.com/search?q=".to_string(),
            enable_javascript: true,
            enable_cookies: true,
        }
    }
}

#[derive(Debug)]
pub struct BrowserState {
    pub tabs: Arc<RwLock<Vec<Tab>>>,
    pub bookmarks: Arc<RwLock<Vec<Bookmark>>>,
    pub bookmark_manager: Arc<RwLock<BookmarkManager>>,
    pub settings: Arc<RwLock<BrowserSettings>>,
    pub history: Arc<RwLock<Vec<String>>>,
    pub webview_navigator: Arc<RwLock<InternalWebView2Navigator>>,
    #[allow(dead_code)]
    #[allow(dead_code)]
    pub proxy_server: Arc<Mutex<Option<crate::proxy_server::ProxyServer>>>,
    pub plugin_manager: Arc<RwLock<crate::plugin_manager::PluginManager>>,
}

impl BrowserState {
    pub fn new() -> Self {
        let bookmark_manager = BookmarkManager::new().expect("Failed to initialize bookmark manager");
        let mut plugin_manager = crate::plugin_manager::PluginManager::new();
        
        // Initialisiere Plugin Manager
        if let Err(e) = plugin_manager.initialize() {
            println!("⚠️ Failed to initialize plugin manager: {}", e);
        }
        
        Self {
            tabs: Arc::new(RwLock::new(vec![])),
            bookmarks: Arc::new(RwLock::new(vec![
                Bookmark {
                    id: "1".to_string(),
                    title: "Google".to_string(),
                    url: "https://www.google.com".to_string(),
                },
                Bookmark {
                    id: "2".to_string(),
                    title: "GitHub".to_string(),
                    url: "https://github.com".to_string(),
                },
            ])),
            bookmark_manager: Arc::new(RwLock::new(bookmark_manager)),
            settings: Arc::new(RwLock::new(BrowserSettings::default())),
            history: Arc::new(RwLock::new(vec![])),
            webview_navigator: Arc::new(RwLock::new(InternalWebView2Navigator::new())),
            proxy_server: Arc::new(Mutex::new(None)),
            plugin_manager: Arc::new(RwLock::new(plugin_manager)),
        }
    }
    
    pub fn save_all_state(&self) {
        // Bookmarks speichern
        let bookmark_manager = self.bookmark_manager.blocking_write();
        if let Err(e) = bookmark_manager.save_bookmarks() {
            println!("❌ Fehler beim Speichern der Bookmarks beim Beenden: {}", e);
        } else {
            println!("💾 Bookmarks beim Beenden erfolgreich gespeichert.");
        }
        // Hier ggf. weitere Persistenz-Logik (z.B. Einstellungen, History) ergänzen
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_browser_state_creation() {
        let state = BrowserState::new();
        
        // Test initial state
        assert!(state.tabs.try_read().is_ok());
        assert!(state.bookmarks.try_read().is_ok());
        assert!(state.settings.try_read().is_ok());
        assert!(state.history.try_read().is_ok());
        
        // Test default bookmarks
        let bookmarks = state.bookmarks.blocking_read();
        assert_eq!(bookmarks.len(), 2);
        assert_eq!(bookmarks[0].title, "Google");
        assert_eq!(bookmarks[1].title, "GitHub");
    }
    
    #[test]
    fn test_browser_settings_default() {
        let settings = BrowserSettings::default();
        
        assert_eq!(settings.homepage, "https://google.com");
        assert_eq!(settings.search_engine, "https://google.com/search?q=");
        assert!(settings.enable_javascript);
        assert!(settings.enable_cookies);
    }
    
    #[test]
    fn test_tab_creation() {
        let tab = Tab {
            id: 1,
            title: "Test Tab".to_string(),
            url: "https://example.com".to_string(),
            is_active: true,
        };
        
        assert_eq!(tab.id, 1);
        assert_eq!(tab.title, "Test Tab");
        assert_eq!(tab.url, "https://example.com");
        assert!(tab.is_active);
    }
    
    #[test]
    fn test_bookmark_creation() {
        let bookmark = Bookmark {
            id: "test-id".to_string(),
            title: "Test Bookmark".to_string(),
            url: "https://test.com".to_string(),
        };
        
        assert_eq!(bookmark.id, "test-id");
        assert_eq!(bookmark.title, "Test Bookmark");
        assert_eq!(bookmark.url, "https://test.com");
    }
} 