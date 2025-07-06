// 📑 Tab Management Commands
// Alle Commands für Tab-Verwaltung

use tauri::Emitter;
use crate::browser_state::{BrowserState, Tab};
use crate::error::OraBrowserError;

/// Erstelle einen neuen Tab
#[tauri::command]
pub async fn create_new_tab(
    state: tauri::State<'_, BrowserState>,
    url: Option<String>,
) -> Result<Tab, OraBrowserError> {
    let mut tabs = state.tabs.write().await;
    
    // Deaktiviere alle existierenden Tabs
    for tab in tabs.iter_mut() {
        tab.is_active = false;
    }
    
    let new_tab = Tab {
        id: uuid::Uuid::new_v4().to_string().parse::<u32>().unwrap(),
        title: "New Tab".to_string(),
        url: url.unwrap_or_else(|| "about:blank".to_string()),
        is_active: true,
    };
    
    tabs.push(new_tab.clone());
    
    println!("📑 New tab created: {}", new_tab.id);
    Ok(new_tab)
}

/// Schließe einen Tab
#[tauri::command]
pub async fn close_tab(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
) -> Result<(), OraBrowserError> {
    let mut tabs = state.tabs.write().await;
    
    if let Some(pos) = tabs.iter().position(|tab| tab.id.to_string() == tab_id) {
        tabs.remove(pos);
        println!("❌ Tab closed: {}", tab_id);
        
        // Aktiviere einen anderen Tab falls verfügbar
        if !tabs.is_empty() && !tabs.iter().any(|tab| tab.is_active) {
            tabs[0].is_active = true;
        }
    }
    
    Ok(())
}

/// Hole alle Tabs
#[tauri::command]
pub async fn get_tabs(state: tauri::State<'_, BrowserState>) -> Result<Vec<Tab>, OraBrowserError> {
    let tabs = state.tabs.read().await;
    Ok(tabs.clone())
}

/// Aktualisiere Tab-Titel
#[tauri::command]
pub async fn update_tab_title(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
    title: String,
) -> Result<(), OraBrowserError> {
    let mut tabs = state.tabs.write().await;
    
    if let Some(tab) = tabs.iter_mut().find(|tab| tab.id.to_string() == tab_id) {
        tab.title = title.clone();
        println!("📝 Tab title updated: {} -> {}", tab_id, title);
    }
    
    Ok(())
}

/// Aktiviere einen bestimmten Tab
#[tauri::command]
pub async fn activate_tab(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
) -> Result<(), OraBrowserError> {
    let mut tabs = state.tabs.write().await;
    
    // Deaktiviere alle Tabs
    for tab in tabs.iter_mut() {
        tab.is_active = false;
    }
    
    // Aktiviere den gewünschten Tab
    if let Some(tab) = tabs.iter_mut().find(|tab| tab.id.to_string() == tab_id) {
        tab.is_active = true;
        println!("🎯 Tab activated: {}", tab_id);
    }
    
    Ok(())
}

/// Duplikiere einen Tab
#[tauri::command]
pub async fn duplicate_tab(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
) -> Result<Tab, OraBrowserError> {
    let mut tabs = state.tabs.write().await;
    
    // Finde Tab-Info zuerst
    let source_info = tabs.iter()
        .find(|tab| tab.id.to_string() == tab_id)
        .map(|tab| (tab.title.clone(), tab.url.clone()));
    
    if let Some((title, url)) = source_info {
        // Deaktiviere alle Tabs
        for tab in tabs.iter_mut() {
            tab.is_active = false;
        }
        
        let new_tab = Tab {
            id: uuid::Uuid::new_v4().to_string().parse::<u32>().unwrap(),
            title: format!("{} (Copy)", title),
            url,
            is_active: true,
        };
        
        tabs.push(new_tab.clone());
        println!("📄 Tab duplicated: {} -> {}", tab_id, new_tab.id);
        Ok(new_tab)
    } else {
        Err(OraBrowserError::ui_error("tab_management", &format!("Tab with ID {} not found", tab_id), false))
    }
}

/// Aktualisiere Tab-URL
#[tauri::command]
pub async fn update_tab_url(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
    url: String,
) -> Result<(), OraBrowserError> {
    let mut tabs = state.tabs.write().await;
    
    if let Some(tab) = tabs.iter_mut().find(|tab| tab.id.to_string() == tab_id) {
        tab.url = url.clone();
        println!("🔗 Tab URL updated: {} -> {}", tab_id, url);
    }
    
    Ok(())
}

/// Hole aktiven Tab
#[tauri::command]
pub async fn get_active_tab(state: tauri::State<'_, BrowserState>) -> Result<Option<Tab>, OraBrowserError> {
    let tabs = state.tabs.read().await;
    Ok(tabs.iter().find(|tab| tab.is_active).cloned())
}

/// Zähle die Anzahl der Tabs
#[tauri::command]
pub async fn get_tab_count(state: tauri::State<'_, BrowserState>) -> Result<usize, OraBrowserError> {
    let tabs = state.tabs.read().await;
    Ok(tabs.len())
}

/// Tab Utilities
pub struct TabUtils;

impl TabUtils {
    /// Erstelle einen eindeutigen Tab-Namen
    pub fn generate_unique_title(base_title: &str, existing_tabs: &[Tab]) -> String {
        let mut counter = 1;
        let mut title = base_title.to_string();
        
        while existing_tabs.iter().any(|tab| tab.title == title) {
            counter += 1;
            title = format!("{} ({})", base_title, counter);
        }
        
        title
    }
    
    /// Validiere Tab-ID
    pub fn is_valid_tab_id(tab_id: &str) -> bool {
        tab_id.parse::<u32>().is_ok()
    }
    
    /// Finde Tab-Index by ID
    pub fn find_tab_index(tabs: &[Tab], tab_id: &str) -> Option<usize> {
        tabs.iter().position(|tab| tab.id.to_string() == tab_id)
    }
} 