use crate::browser_state::BrowserState;
use crate::plugin::PluginInfo;
use crate::error::OraBrowserError;

#[tauri::command]
pub async fn get_all_plugins(
    state: tauri::State<'_, BrowserState>,
) -> Result<Vec<PluginInfo>, OraBrowserError> {
    let plugin_manager = state.plugin_manager.read().await;
    Ok(plugin_manager.get_all_plugins().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn get_loaded_plugins(
    state: tauri::State<'_, BrowserState>,
) -> Result<Vec<PluginInfo>, OraBrowserError> {
    let plugin_manager = state.plugin_manager.read().await;
    Ok(plugin_manager.get_loaded_plugins().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn enable_plugin(
    state: tauri::State<'_, BrowserState>,
    plugin_id: String,
) -> Result<(), OraBrowserError> {
    let mut plugin_manager = state.plugin_manager.write().await;
    plugin_manager.enable_plugin(&plugin_id)
        .map_err(|e| OraBrowserError::plugin_error(&plugin_id, &format!("Failed to enable plugin: {}", e)))
}

#[tauri::command]
pub async fn disable_plugin(
    state: tauri::State<'_, BrowserState>,
    plugin_id: String,
) -> Result<(), OraBrowserError> {
    let mut plugin_manager = state.plugin_manager.write().await;
    plugin_manager.disable_plugin(&plugin_id)
        .map_err(|e| OraBrowserError::plugin_error(&plugin_id, &format!("Failed to disable plugin: {}", e)))
}

#[tauri::command]
pub async fn load_plugin(
    state: tauri::State<'_, BrowserState>,
    plugin_id: String,
) -> Result<(), OraBrowserError> {
    let mut plugin_manager = state.plugin_manager.write().await;
    plugin_manager.load_plugin(&plugin_id)
        .map_err(|e| OraBrowserError::plugin_error(&plugin_id, &format!("Failed to load plugin: {}", e)))
}

#[tauri::command]
pub async fn unload_plugin(
    state: tauri::State<'_, BrowserState>,
    plugin_id: String,
) -> Result<(), OraBrowserError> {
    let mut plugin_manager = state.plugin_manager.write().await;
    plugin_manager.unload_plugin(&plugin_id)
        .map_err(|e| OraBrowserError::plugin_error(&plugin_id, &format!("Failed to unload plugin: {}", e)))
} 