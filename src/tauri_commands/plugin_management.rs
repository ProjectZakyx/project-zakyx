use crate::browser_state::BrowserState;
use crate::plugin::PluginInfo;
use crate::error::ZAKYXBrowserError;

#[tauri::command]
pub async fn get_all_plugins(
    state: tauri::State<'_, BrowserState>,
) -> Result<Vec<PluginInfo>, ZAKYXBrowserError> {
    let _plugin_manager = state.plugin_manager.read().await;
    // TODO: Implement proper plugin listing after PluginManager refactoring
    Ok(vec![])
}

#[tauri::command]
pub async fn get_loaded_plugins(
    state: tauri::State<'_, BrowserState>,
) -> Result<Vec<PluginInfo>, ZAKYXBrowserError> {
    let _plugin_manager = state.plugin_manager.read().await;
    // TODO: Implement proper loaded plugin listing after PluginManager refactoring
    Ok(vec![])
}

#[tauri::command]
pub async fn enable_plugin(
    state: tauri::State<'_, BrowserState>,
    plugin_id: String,
) -> Result<(), ZAKYXBrowserError> {
    let mut plugin_manager = state.plugin_manager.write().await;
    plugin_manager.enable_plugin(&plugin_id)
        .map_err(|e| ZAKYXBrowserError::plugin_error(&plugin_id, &format!("Failed to enable plugin: {}", e)))
}

#[tauri::command]
pub async fn disable_plugin(
    state: tauri::State<'_, BrowserState>,
    plugin_id: String,
) -> Result<(), ZAKYXBrowserError> {
    let mut plugin_manager = state.plugin_manager.write().await;
    plugin_manager.disable_plugin(&plugin_id)
        .map_err(|e| ZAKYXBrowserError::plugin_error(&plugin_id, &format!("Failed to disable plugin: {}", e)))
}

#[tauri::command]
pub async fn load_plugin(
    state: tauri::State<'_, BrowserState>,
    plugin_id: String,
) -> Result<(), ZAKYXBrowserError> {
    let mut plugin_manager = state.plugin_manager.write().await;
    plugin_manager.load_plugin(&plugin_id)
        .map_err(|e| ZAKYXBrowserError::plugin_error(&plugin_id, &format!("Failed to load plugin: {}", e)))
}

#[tauri::command]
pub async fn unload_plugin(
    state: tauri::State<'_, BrowserState>,
    plugin_id: String,
) -> Result<(), ZAKYXBrowserError> {
    let mut plugin_manager = state.plugin_manager.write().await;
    plugin_manager.unload_plugin(&plugin_id)
        .map_err(|e| ZAKYXBrowserError::plugin_error(&plugin_id, &format!("Failed to unload plugin: {}", e)))
} 
