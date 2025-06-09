// 🌐 ORA BROWSER - TAURI v2 EDITION
// Modern Cross-Platform Web Browser built with Rust + Tauri
// Copyright © 2024 Ora Browser Team

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Emitter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// 🗂️ BROWSER STATE & DATA STRUCTURES
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: String,
    pub title: String,
    pub url: String,
    pub favicon: Option<String>,
    pub is_active: bool,
    pub is_loading: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub title: String,
    pub url: String,
    pub favicon: Option<String>,
    pub folder: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSettings {
    pub homepage: String,
    pub search_engine: String,
    pub privacy_mode: bool,
    pub ad_blocker: bool,
    pub javascript_enabled: bool,
    pub cookies_enabled: bool,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            homepage: "https://www.google.com".to_string(),
            search_engine: "https://www.google.com/search?q=".to_string(),
            privacy_mode: false,
            ad_blocker: true,
            javascript_enabled: true,
            cookies_enabled: true,
        }
    }
}

#[derive(Debug, Default)]
pub struct BrowserState {
    pub tabs: Arc<RwLock<Vec<Tab>>>,
    pub bookmarks: Arc<RwLock<Vec<Bookmark>>>,
    pub settings: Arc<RwLock<BrowserSettings>>,
    pub history: Arc<RwLock<Vec<String>>>,
}

// 🎯 TAURI COMMANDS

#[tauri::command]
async fn create_new_tab(
    state: tauri::State<'_, BrowserState>,
    url: Option<String>,
) -> Result<Tab, String> {
    let mut tabs = state.tabs.write().await;
    
    // Deactivate all existing tabs
    for tab in tabs.iter_mut() {
        tab.is_active = false;
    }
    
    let new_tab = Tab {
        id: uuid::Uuid::new_v4().to_string(),
        title: "New Tab".to_string(),
        url: url.unwrap_or_else(|| "about:blank".to_string()),
        favicon: None,
        is_active: true,
        is_loading: false,
    };
    
    tabs.push(new_tab.clone());
    
    println!("📑 New tab created: {}", new_tab.id);
    Ok(new_tab)
}

#[tauri::command]
async fn close_tab(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
) -> Result<(), String> {
    let mut tabs = state.tabs.write().await;
    
    if let Some(pos) = tabs.iter().position(|tab| tab.id == tab_id) {
        tabs.remove(pos);
        println!("❌ Tab closed: {}", tab_id);
        
        // Activate another tab if available
        if !tabs.is_empty() && !tabs.iter().any(|tab| tab.is_active) {
            tabs[0].is_active = true;
        }
    }
    
    Ok(())
}

#[tauri::command]
async fn navigate_to(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
    url: String,
    window: tauri::Window,
) -> Result<(), String> {
    let mut tabs = state.tabs.write().await;
    let mut history = state.history.write().await;
    
    if let Some(tab) = tabs.iter_mut().find(|tab| tab.id == tab_id) {
        tab.url = url.clone();
        tab.is_loading = true;
        tab.title = "Loading...".to_string();
        
        // Add to history
        history.push(url.clone());
        
        // Emit event to frontend to actually load the URL
        window.emit("webview_navigate", &url).map_err(|e| e.to_string())?;
        
        println!("🌐 Navigating tab {} to: {}", tab_id, url);
        
        // Simulate loading completion after a short delay
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let _ = window.emit("webview_loaded", &tab_id);
        });
    }
    
    Ok(())
}

#[tauri::command]
async fn get_tabs(state: tauri::State<'_, BrowserState>) -> Result<Vec<Tab>, String> {
    let tabs = state.tabs.read().await;
    Ok(tabs.clone())
}

#[tauri::command]
async fn add_bookmark(
    state: tauri::State<'_, BrowserState>,
    title: String,
    url: String,
) -> Result<Bookmark, String> {
    let mut bookmarks = state.bookmarks.write().await;
    
    let bookmark = Bookmark {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        url,
        favicon: None,
        folder: None,
    };
    
    bookmarks.push(bookmark.clone());
    println!("⭐ Bookmark added: {}", bookmark.title);
    
    Ok(bookmark)
}

#[tauri::command]
async fn get_bookmarks(state: tauri::State<'_, BrowserState>) -> Result<Vec<Bookmark>, String> {
    let bookmarks = state.bookmarks.read().await;
    Ok(bookmarks.clone())
}

#[tauri::command]
async fn remove_bookmark(
    state: tauri::State<'_, BrowserState>,
    bookmark_id: String,
) -> Result<(), String> {
    let mut bookmarks = state.bookmarks.write().await;
    
    if let Some(pos) = bookmarks.iter().position(|b| b.id == bookmark_id) {
        let bookmark = bookmarks.remove(pos);
        println!("🗑️ Bookmark removed: {}", bookmark.title);
    }
    
    Ok(())
}

#[tauri::command]
async fn get_settings(state: tauri::State<'_, BrowserState>) -> Result<BrowserSettings, String> {
    let settings = state.settings.read().await;
    Ok(settings.clone())
}

#[tauri::command]
async fn update_settings(
    state: tauri::State<'_, BrowserState>,
    new_settings: BrowserSettings,
) -> Result<(), String> {
    let mut settings = state.settings.write().await;
    *settings = new_settings;
    println!("⚙️ Settings updated");
    Ok(())
}

#[tauri::command]
async fn open_external_url(url: String) -> Result<(), String> {
    println!("🌐 Opening external URL: {}", url);
    
    // Use std::process to open URL in system browser
    #[cfg(target_os = "windows")]
    let result = std::process::Command::new("cmd")
        .args(&["/C", "start", &url])
        .spawn();
    
    #[cfg(target_os = "macos")]
    let result = std::process::Command::new("open")
        .arg(&url)
        .spawn();
    
    #[cfg(target_os = "linux")]
    let result = std::process::Command::new("xdg-open")
        .arg(&url)
        .spawn();
    
    match result {
        Ok(_) => {
            println!("✅ Successfully opened URL in system browser");
            Ok(())
        },
        Err(e) => {
            println!("❌ Failed to open URL: {}", e);
            Err(format!("Failed to open URL: {}", e))
        }
    }
}

#[tauri::command]
async fn get_history(state: tauri::State<'_, BrowserState>) -> Result<Vec<String>, String> {
    let history = state.history.read().await;
    Ok(history.clone())
}

#[tauri::command]
async fn update_tab_title(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
    title: String,
) -> Result<(), String> {
    let mut tabs = state.tabs.write().await;
    
    if let Some(tab) = tabs.iter_mut().find(|tab| tab.id == tab_id) {
        tab.title = title;
        tab.is_loading = false;
        println!("📄 Tab {} title updated: {}", tab_id, tab.title);
    }
    
    Ok(())
}

// 🚀 MAIN FUNCTION - TAURI v2
fn main() {
    // Initialize default browser state
    let default_settings = BrowserSettings {
        homepage: "https://www.google.com".to_string(),
        search_engine: "https://www.google.com/search?q=".to_string(),
        privacy_mode: false,
        ad_blocker: true,
        javascript_enabled: true,
        cookies_enabled: true,
    };
    
    let state = BrowserState {
        tabs: Arc::new(RwLock::new(vec![])),
        bookmarks: Arc::new(RwLock::new(vec![
            Bookmark {
                id: "1".to_string(),
                title: "Google".to_string(),
                url: "https://www.google.com".to_string(),
                favicon: None,
                folder: None,
            },
            Bookmark {
                id: "2".to_string(),
                title: "GitHub".to_string(),
                url: "https://github.com".to_string(),
                favicon: None,
                folder: None,
            },
        ])),
        settings: Arc::new(RwLock::new(default_settings)),
        history: Arc::new(RwLock::new(vec![])),
    };
    
    println!("🚀 Starting Ora Browser with Tauri v2...");
    
    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            create_new_tab,
            close_tab,
            navigate_to,
            get_tabs,
            add_bookmark,
            get_bookmarks,
            remove_bookmark,
            get_settings,
            update_settings,
            open_external_url,
            get_history,
            update_tab_title
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            
            // Set window properties
            window.set_title("Ora Browser").unwrap();
            
            println!("✅ Ora Browser window created successfully!");
            println!("🌐 Ready for cross-platform browsing!");
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
